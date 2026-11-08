// language: Rust, bin: axis-gui, target: Windows 10/11, MSVC
// v6.0 — rebrand profissional (SENTINEL engine, categorias neutras). Proxy pro engine Python
// em 127.0.0.1:8899 (sentinel_engine.py) → backend dos módulos `defense --action sentinel-*`.
use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

const SENTINEL_URL: &str = "http://127.0.0.1:8899";

// ═══════════════════════════════════════════════════════════════════════
// EMBEDDED SPA
// ═══════════════════════════════════════════════════════════════════════
const INDEX_HTML: &str = r##"<!doctype html>
<html lang="pt-BR">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>AXIS v6.0 · Security Platform</title>
<style>
:root{
  --bg:#08080c;--surface:#0e0e14;--surface-2:#14141c;--surface-3:#1a1a24;
  --border:#202030;--border-2:#2a2a3a;
  --text:#c8c8d4;--text-dim:#6e6e80;--text-bright:#f0f0f8;
  --cyan:#00e5ff;--magenta:#ff00e5;--green:#00e676;--yellow:#ffc400;--red:#ff1744;
  --mono:'Cascadia Code','Consolas','Courier New',monospace;
}
*{box-sizing:border-box;margin:0;padding:0}
html,body{height:100%}
body{font-family:var(--mono);background:var(--bg);color:var(--text);font-size:13px;overflow:hidden}

.login{position:fixed;inset:0;display:flex;align-items:center;justify-content:center;
  background:var(--bg);z-index:200;overflow:hidden}
.login::before{content:'';position:absolute;inset:-20%;pointer-events:none;background:
  radial-gradient(circle at 30% 30%,rgba(0,229,255,.09),transparent 45%),
  radial-gradient(circle at 70% 70%,rgba(255,0,229,.09),transparent 45%);
  animation:drift 22s ease-in-out infinite}
@keyframes drift{0%,100%{transform:translate(0,0) scale(1)}50%{transform:translate(2%,-2%) scale(1.06)}}
.login::after{content:'';position:absolute;inset:0;pointer-events:none;
  background:repeating-linear-gradient(0deg,transparent 0,transparent 3px,
  rgba(0,229,255,.014) 3px,rgba(0,229,255,.014) 4px)}
.login-card{position:relative;z-index:1;width:100%;max-width:400px;padding:40px 36px;
  background:rgba(14,14,20,.88);border:1px solid var(--border-2);border-radius:8px;
  backdrop-filter:blur(10px);box-shadow:0 20px 80px rgba(0,0,0,.6),0 0 1px rgba(0,229,255,.4);
  animation:fadeIn .5s ease}
@keyframes fadeIn{from{opacity:0;transform:translateY(10px)}to{opacity:1;transform:none}}
.login.leaving{pointer-events:none}
.login.leaving .login-card{opacity:0;transform:translateY(-44vh)}
.login-logo{display:flex;justify-content:center;margin-bottom:10px}
.login-logo svg{animation:breathe 4s ease-in-out infinite}
@keyframes breathe{
  0%,100%{transform:scale(1);filter:drop-shadow(0 0 8px rgba(0,229,255,.35))}
  50%{transform:scale(1.04);filter:drop-shadow(0 0 16px rgba(255,0,229,.5))}}
.login-brand{text-align:center;font-size:34px;font-weight:600;letter-spacing:10px;margin-bottom:4px;
  background:linear-gradient(90deg,var(--cyan),var(--magenta));
  -webkit-background-clip:text;background-clip:text;-webkit-text-fill-color:transparent}
.login-sub{text-align:center;font-size:9px;color:var(--text-dim);letter-spacing:3px;
  text-transform:uppercase;margin-bottom:22px}
.login-boot{font-size:11px;color:var(--text-dim);line-height:1.65;min-height:78px;
  margin-bottom:16px;padding:10px 12px;background:var(--surface-2);
  border:1px solid var(--border);border-radius:4px;white-space:pre-wrap}
.login-boot .ok{color:var(--green)}
.boot-cursor{display:inline-block;width:6px;animation:blink 1s steps(1) infinite}
@keyframes blink{50%{opacity:0}}
.login-field{margin-bottom:12px}
.login-field label{display:block;font-size:9px;color:var(--text-dim);letter-spacing:2px;
  text-transform:uppercase;margin-bottom:5px}
.login-field input{width:100%;padding:10px 12px;background:var(--surface-2);
  border:1px solid var(--border);color:var(--text-bright);font-family:var(--mono);
  font-size:13px;border-radius:4px;outline:none}
.login-field input:focus{border-color:var(--cyan);box-shadow:0 0 0 1px rgba(0,229,255,.15)}
.login-btn{width:100%;padding:12px;margin-top:6px;
  background:linear-gradient(90deg,rgba(0,229,255,.12),rgba(255,0,229,.12));
  border:1px solid var(--cyan);color:var(--cyan);font-family:var(--mono);
  font-size:11px;letter-spacing:4px;font-weight:600;border-radius:4px;cursor:pointer}
.login-btn:hover{background:linear-gradient(90deg,rgba(0,229,255,.22),rgba(255,0,229,.22));
  box-shadow:0 0 24px rgba(0,229,255,.35)}
.login-meta{display:flex;justify-content:center;gap:6px;margin-top:16px;
  font-size:9px;color:var(--text-dim);letter-spacing:1px}
.login-footer{position:absolute;bottom:14px;left:0;right:0;text-align:center;
  font-size:9px;color:var(--text-dim);letter-spacing:1.5px}

.app{display:grid;height:100vh;grid-template-columns:262px 1fr;
  grid-template-rows:48px 1fr 26px;
  grid-template-areas:"header header" "sidebar main" "footer footer";
  opacity:0;transition:opacity .4s .35s}
.app[hidden]{display:none}
.app.entering{opacity:1}
.app header{grid-area:header;display:flex;align-items:center;gap:14px;padding:0 14px;
  background:var(--surface);border-bottom:1px solid var(--border)}
.brand{display:flex;align-items:center;gap:8px;font-size:13px;font-weight:600;letter-spacing:2px}
.brand-name{background:linear-gradient(90deg,var(--cyan),var(--magenta));
  -webkit-background-clip:text;background-clip:text;-webkit-text-fill-color:transparent}
.status{display:flex;gap:6px;margin-left:4px}
.pill{font-size:9px;padding:2px 7px;border:1px solid var(--border);border-radius:3px;
  background:var(--surface-2);letter-spacing:.5px;display:inline-flex;align-items:center;gap:5px;
  cursor:pointer;transition:border-color .12s,background .12s}
.pill:hover{border-color:var(--cyan);background:var(--surface-3)}
.pill .dot{width:5px;height:5px;border-radius:50%;background:var(--text-dim)}
.pill.ok .dot{background:var(--green);box-shadow:0 0 5px var(--green)}
.pill.warn .dot{background:var(--yellow);box-shadow:0 0 5px var(--yellow)}
.pill.bad .dot{background:var(--red);box-shadow:0 0 5px var(--red)}
.pill .label{color:var(--text-dim)}
.pill .value{color:var(--text)}
.header-spacer{flex:1}
.hdr-btn{font-size:9px;color:var(--text-dim);cursor:pointer;padding:4px 9px;
  border:1px solid var(--border);border-radius:3px;background:transparent;
  font-family:var(--mono);letter-spacing:1.5px;text-transform:uppercase;transition:all .12s}
.hdr-btn:hover{color:var(--cyan);border-color:var(--cyan)}
.hdr-btn.danger:hover{color:var(--red);border-color:var(--red)}
.hdr-btn.active{color:var(--cyan);border-color:var(--cyan);
  background:rgba(0,229,255,.06);box-shadow:0 0 12px rgba(0,229,255,.15)}

.sidebar{grid-area:sidebar;background:var(--surface);border-right:1px solid var(--border);
  display:flex;flex-direction:column;overflow:hidden}
.search-wrap{padding:8px;border-bottom:1px solid var(--border)}
.search{width:100%;padding:6px 10px;background:var(--surface-2);border:1px solid var(--border);
  color:var(--text);font-family:var(--mono);font-size:11px;border-radius:3px;outline:none}
.search:focus{border-color:var(--cyan)}
.recents{padding:6px 8px 8px;border-bottom:1px solid var(--border);
  background:linear-gradient(180deg,rgba(0,229,255,.03),transparent)}
.recents:empty{display:none}
.recents-label{font-size:9px;letter-spacing:1.5px;text-transform:uppercase;
  color:var(--text-dim);padding:0 4px 4px}
.recent-item{display:flex;align-items:center;gap:8px;padding:5px 8px;border-radius:3px;
  cursor:pointer;font-size:11px;color:var(--text);transition:background .12s}
.recent-item:hover{background:var(--surface-2)}
.recent-item .kbd{margin-left:auto;font-size:9px;color:var(--text-dim);
  border:1px solid var(--border);border-radius:2px;padding:0 4px}
.cat-list{flex:1;overflow-y:auto}
.cat-list::-webkit-scrollbar{width:6px}
.cat-list::-webkit-scrollbar-thumb{background:var(--surface-3);border-radius:3px}
.cat{border-bottom:1px solid var(--border)}
.cat-header{display:flex;align-items:center;gap:8px;padding:8px 12px;cursor:pointer;
  font-size:9px;text-transform:uppercase;letter-spacing:2px;color:var(--text-dim);
  user-select:none;transition:background .12s}
.cat-header:hover{background:var(--surface-2);color:var(--text)}
.cat-dot{width:6px;height:6px;border-radius:50%;flex-shrink:0}
.cat-name{flex:1}
.cat-count{font-size:9px;color:var(--text-dim)}
.cat-chevron{font-size:8px;transition:transform .15s;color:var(--text-dim)}
.cat.open .cat-chevron{transform:rotate(90deg)}
.cat-body{display:none}
.cat.open .cat-body{display:block;padding-bottom:4px}
.mod-item{position:relative;padding:5px 12px 5px 26px;font-size:12px;cursor:pointer;
  border-left:2px solid transparent;color:var(--text);line-height:1.35}
.mod-item::before{content:'';position:absolute;left:0;top:0;bottom:0;width:2px;
  background:var(--cyan);transform:scaleY(0);transform-origin:center;
  transition:transform .18s cubic-bezier(.16,1,.3,1)}
.mod-item:hover{background:var(--surface-2)}
.mod-item.active{background:var(--surface-2);color:var(--cyan);border-left-color:transparent}
.mod-item.active::before{transform:scaleY(1)}
.mod-item .irrev{color:var(--red);font-size:9px;margin-left:5px}

.main{grid-area:main;display:flex;flex-direction:column;overflow:hidden;background:var(--bg)}
.main[hidden]{display:none}
.pane{padding:16px 20px;border-bottom:1px solid var(--border);background:var(--surface);flex-shrink:0}
.pane h2{font-size:13px;font-weight:600;color:var(--text-bright);margin-bottom:14px;letter-spacing:.4px}
.breadcrumb{display:inline}
.bc-cat{color:var(--text-dim);font-size:10px;letter-spacing:1.5px;text-transform:uppercase;
  cursor:pointer;transition:color .12s}
.bc-cat:hover{color:var(--cyan)}
.bc-sep{color:var(--text-dim);margin:0 6px}
.bc-mod{color:var(--text-bright)}
.irrev-tag{color:var(--red);font-size:10px;letter-spacing:1.5px;margin-left:6px}
.form-grid{display:grid;grid-template-columns:160px 1fr;gap:8px 12px;max-width:720px;align-items:center}
.form-grid label{color:var(--text-dim);font-size:11px}
.form-grid input,.form-grid select{background:var(--surface-2);border:1px solid var(--border);
  color:var(--text);font-family:var(--mono);font-size:12px;padding:7px 10px;border-radius:3px;
  width:100%;outline:none}
.form-grid input:focus,.form-grid select:focus{border-color:var(--cyan);
  box-shadow:0 0 0 1px rgba(0,229,255,.15)}
.field-hint{grid-column:2;font-size:10px;color:var(--text-dim);margin-top:-4px;padding:0 2px}
.actions{display:flex;gap:8px;margin-top:16px}
button.action{position:relative;overflow:hidden;font-family:var(--mono);font-size:11px;
  padding:7px 18px;border-radius:3px;border:1px solid var(--border);background:var(--surface-2);
  color:var(--text);cursor:pointer;letter-spacing:1.5px;transition:all .12s}
button.action:hover{border-color:var(--cyan);color:var(--cyan)}
button.action.primary{background:linear-gradient(90deg,rgba(0,229,255,.12),rgba(255,0,229,.12));
  border-color:var(--cyan);color:var(--cyan);font-weight:600}
button.action.danger{border-color:var(--red);color:var(--red)}
button.action.danger:hover{background:rgba(255,23,68,.15)}
button.action.loading{pointer-events:none;opacity:.7}
button.action:disabled{opacity:.4;cursor:not-allowed}
.output{flex:1;min-height:160px;overflow-y:auto;padding:12px 20px;background:var(--bg)}
.output-header{display:flex;justify-content:space-between;align-items:center;
  font-size:9px;color:var(--text-dim);letter-spacing:1.5px;text-transform:uppercase;margin-bottom:8px}
.out-tabs{display:flex;gap:2px}
.out-tab{font:inherit;font-size:9px;letter-spacing:1.5px;text-transform:uppercase;
  padding:3px 9px;background:transparent;border:1px solid transparent;border-radius:3px;
  color:var(--text-dim);cursor:pointer}
.out-tab.active{color:var(--cyan);border-color:var(--border-2);background:var(--surface-2)}
.out-actions{display:flex;align-items:center;gap:6px}
.out-act{font:inherit;font-size:9px;letter-spacing:1px;text-transform:uppercase;
  color:var(--text-dim);background:transparent;border:1px solid var(--border);
  border-radius:3px;padding:3px 8px;cursor:pointer}
.out-act:hover{color:var(--cyan);border-color:var(--cyan)}
.output pre{white-space:pre-wrap;word-break:break-word;font-size:11px;line-height:1.55;color:var(--text)}
.app footer{grid-area:footer;border-top:1px solid var(--border);background:var(--surface);
  display:flex;gap:10px;align-items:center;padding:0 14px;font-size:9px;
  color:var(--text-dim);letter-spacing:.5px}
.footer-spacer{flex:1}

.modal-backdrop{position:fixed;inset:0;background:rgba(0,0,0,.78);display:none;
  align-items:center;justify-content:center;z-index:150;backdrop-filter:blur(4px)}
.modal-backdrop.show{display:flex}
.modal{background:var(--surface);border:1px solid var(--red);border-radius:6px;
  padding:24px;max-width:520px;width:90%;box-shadow:0 20px 60px rgba(255,23,68,.25)}
.modal h3{color:var(--red);font-size:12px;margin-bottom:12px;letter-spacing:2px;text-transform:uppercase}
.modal p{color:var(--text);font-size:11px;line-height:1.65;margin-bottom:12px}
.modal code{display:block;background:var(--surface-2);padding:8px 12px;border-radius:3px;
  color:var(--cyan);font-size:10px;word-break:break-all;margin-bottom:16px;border:1px solid var(--border)}
.modal .actions{justify-content:flex-end;margin-top:0}
.confirm-input{width:100%;padding:8px 10px;background:var(--surface-2);border:1px solid var(--border);
  border-radius:3px;color:var(--text-bright);font:12px var(--mono);margin:8px 0 16px;
  letter-spacing:2px;outline:none}
.confirm-input:focus{border-color:var(--red);box-shadow:0 0 0 1px rgba(255,23,68,.2)}

.toasts{position:fixed;bottom:44px;right:16px;display:flex;flex-direction:column;gap:6px;
  z-index:300;pointer-events:none}
.toast{pointer-events:auto;background:var(--surface-2);border:1px solid var(--border-2);
  border-left:2px solid var(--cyan);border-radius:4px;padding:8px 14px;
  font-size:11px;color:var(--text);box-shadow:0 8px 24px rgba(0,0,0,.5);
  animation:toastIn .25s cubic-bezier(.16,1,.3,1)}
.toast.warn{border-left-color:var(--yellow)}
.toast.err{border-left-color:var(--red)}
.toast.aura{border-left-color:var(--magenta);box-shadow:0 8px 24px rgba(255,0,229,.35)}
.toast.out{animation:toastOut .2s ease forwards}
@keyframes toastIn{from{opacity:0;transform:translateX(20px)}}
@keyframes toastOut{to{opacity:0;transform:translateX(20px)}}

.palette-backdrop{position:fixed;inset:0;background:rgba(0,0,0,.6);backdrop-filter:blur(3px);
  z-index:250;display:none;align-items:flex-start;justify-content:center;padding-top:14vh}
.palette-backdrop.show{display:flex;animation:fadeIn .15s ease}
.palette{width:min(520px,90vw);background:var(--surface);border:1px solid var(--border-2);
  border-radius:6px;box-shadow:0 24px 80px rgba(0,0,0,.7);overflow:hidden}
.palette-input{width:100%;padding:14px 18px;background:var(--surface-2);border:0;
  border-bottom:1px solid var(--border);color:var(--text-bright);font:13px var(--mono);outline:none}
.palette-list{max-height:340px;overflow-y:auto}
.palette-item{padding:9px 18px;font-size:12px;color:var(--text);cursor:pointer;
  display:flex;align-items:center;gap:10px;border-left:2px solid transparent}
.palette-item.sel{background:var(--surface-2);color:var(--cyan);border-left-color:var(--cyan)}
.palette-item .cat-dot{flex-shrink:0}
.palette-item .cat-name{margin-left:auto;font-size:9px;color:var(--text-dim);
  letter-spacing:1px;text-transform:uppercase}

.docs-view,.config-view,.apis-view,.cli-view{overflow:hidden}
.docs-wrap,.config-wrap{max-width:820px;margin:0 auto;line-height:1.65}
.docs-h1,.config-h1{font-size:22px;margin-bottom:6px;letter-spacing:.5px;
  background:linear-gradient(90deg,var(--cyan),var(--magenta));
  -webkit-background-clip:text;background-clip:text;-webkit-text-fill-color:transparent}
.docs-lead,.config-lead{color:var(--text-dim);font-size:12px;margin-bottom:24px}
.docs-h2{font-size:12px;color:var(--cyan);margin:26px 0 10px;letter-spacing:2px;
  text-transform:uppercase;border-bottom:1px solid var(--border);padding-bottom:6px}
.docs-p{color:var(--text);font-size:12px;margin:8px 0}
.docs-ol,.docs-ul{padding-left:20px;margin:8px 0;font-size:12px}
.docs-ol li,.docs-ul li{margin:4px 0;color:var(--text)}
.docs-pre{background:var(--surface-2);border:1px solid var(--border);border-radius:4px;
  padding:10px 12px;font-size:11px;color:var(--text);margin:8px 0;overflow-x:auto;white-space:pre}
.docs-table{width:100%;border-collapse:collapse;font-size:11px;margin:8px 0}
.docs-table th{text-align:left;color:var(--cyan);padding:6px 10px;
  border-bottom:1px solid var(--border);font-weight:600}
.docs-table td{padding:6px 10px;border-bottom:1px solid var(--border);color:var(--text)}
.docs-table code{color:var(--yellow);font-size:10px}
.docs-cats{display:grid;grid-template-columns:repeat(auto-fill,minmax(180px,1fr));gap:4px;margin:10px 0}
.docs-cat{padding:6px 10px;background:var(--surface-2);border:1px solid var(--border);
  border-radius:3px;font-size:11px;display:flex;align-items:center;gap:8px}
.docs-cat .swatch{width:8px;height:8px;border-radius:50%;flex-shrink:0}
.docs-cat .count{margin-left:auto;color:var(--text-dim);font-size:10px}
.docs-warn{background:rgba(255,196,0,.08);border:1px solid rgba(255,196,0,.3);
  border-radius:4px;padding:10px 14px;margin:12px 0;font-size:11px;color:var(--text)}
.docs-warn b{color:var(--yellow)}

.config-section{margin-bottom:26px;padding-bottom:20px;border-bottom:1px solid var(--border)}
.config-section:last-child{border-bottom:none}
.config-section h2{font-size:11px;color:var(--cyan);letter-spacing:2px;
  text-transform:uppercase;margin-bottom:12px}
.config-fields{display:flex;flex-direction:column;gap:10px}
.config-field{display:grid;grid-template-columns:200px 1fr;gap:10px;align-items:center}
.config-field label{font-size:11px;color:var(--text-dim)}
.config-field input,.config-field textarea{background:var(--surface-2);border:1px solid var(--border);
  color:var(--text-bright);font:12px var(--mono);padding:7px 10px;border-radius:3px;
  width:100%;outline:none}
.config-field input:focus,.config-field textarea:focus{border-color:var(--cyan);
  box-shadow:0 0 0 1px rgba(0,229,255,.15)}
.config-field input[readonly]{color:var(--text-dim);cursor:not-allowed;
  background:var(--surface);border-style:dashed}
.cfg-hint{grid-column:2;font-size:10px;color:var(--text-dim);margin-top:-6px;padding:0 2px}
.cfg-procs{display:flex;flex-wrap:wrap;gap:6px;font-size:11px}
.cfg-proc{background:var(--surface-2);border:1px solid var(--border);border-radius:3px;
  padding:3px 8px;color:var(--cyan)}
.config-scope{width:100%;min-height:180px;background:var(--surface-2);
  border:1px solid var(--border);color:var(--text-bright);font:11px var(--mono);
  padding:10px 12px;border-radius:3px;resize:vertical;outline:none;white-space:pre;overflow:auto}
.config-scope:focus{border-color:var(--cyan);box-shadow:0 0 0 1px rgba(0,229,255,.15)}
.config-actions{display:flex;flex-wrap:wrap;gap:8px;margin-top:8px}
.config-p{font-size:11px;color:var(--text);line-height:1.6;margin-bottom:10px}
.cfg-pill{display:inline-flex;align-items:center;gap:5px;padding:2px 7px;
  border:1px solid var(--border);border-radius:3px;background:var(--surface-2);
  font-size:10px;letter-spacing:.3px}
.cfg-pill.ok{border-color:var(--green);color:var(--green)}
.cfg-pill.bad{border-color:var(--red);color:var(--red)}
.cfg-status{display:flex;gap:14px;align-items:center;font-size:11px;
  padding:10px 14px;background:rgba(0,229,255,.04);border:1px solid var(--border);border-radius:4px}

.apis-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(320px,1fr));gap:14px;margin-top:8px}
.apis-card{background:var(--surface-2);border:1px solid var(--border);border-radius:5px;
  padding:14px 16px;display:flex;flex-direction:column;gap:8px}
.apis-card h3{font-size:10px;letter-spacing:2px;text-transform:uppercase;color:var(--cyan);
  display:flex;align-items:center;gap:8px}
.apis-card .bind-status{font-size:9px;color:var(--text-dim);letter-spacing:1px;margin-left:auto}
.apis-card .bind-status.ok{color:var(--green)}
.apis-card .bind-status.bad{color:var(--red)}
.apis-card input{background:var(--surface);border:1px solid var(--border);
  color:var(--text-bright);font:11px var(--mono);padding:6px 9px;border-radius:3px;outline:none}
.apis-card input:focus{border-color:var(--cyan);box-shadow:0 0 0 1px rgba(0,229,255,.15)}
.apis-card .apis-actions{display:flex;gap:6px;margin-top:2px}
.apis-card .apis-hint{font-size:10px;color:var(--text-dim);line-height:1.5}

.cli-wrap{display:flex;flex-direction:column;height:calc(100vh - 48px - 26px);
  padding:16px 20px;gap:10px;max-width:1000px;margin:0 auto;width:100%}
.cli-head{display:flex;justify-content:space-between;align-items:baseline;gap:12px}
.cli-h1{font-size:20px;letter-spacing:.5px;
  background:linear-gradient(90deg,var(--cyan),var(--magenta));
  -webkit-background-clip:text;background-clip:text;-webkit-text-fill-color:transparent}
.cli-lead{font-size:11px;color:var(--text-dim)}
.cli-console{flex:1;background:var(--surface);border:1px solid var(--border);
  border-radius:6px;display:flex;flex-direction:column;overflow:hidden}
.cli-out{flex:1;padding:14px 16px;overflow-y:auto;color:var(--text);
  line-height:1.6;white-space:pre-wrap;word-break:break-word;font-size:12px}
.cli-line{display:block;margin-bottom:2px}
.cli-line.echo{color:var(--cyan);font-weight:600}
.cli-line.echo::before{content:'$ ';color:var(--green)}
.cli-line.ok{color:var(--text)}
.cli-line.dim{color:var(--text-dim)}
.cli-line.err{color:var(--red)}
.cli-line.warn{color:var(--yellow)}
.cli-line.aura{color:var(--magenta);text-shadow:0 0 8px rgba(255,0,229,.35);font-weight:600}
.cli-line.aura::before{content:'✦ ';color:var(--magenta)}
.cli-in-wrap{display:flex;align-items:center;gap:6px;padding:9px 14px;
  border-top:1px solid var(--border);background:var(--surface-2)}
.cli-prompt{color:var(--green);font-weight:600}
.cli-input{flex:1;background:transparent;border:0;color:var(--text-bright);
  font:12px var(--mono);outline:none;caret-color:var(--cyan)}
.cli-suggest{display:none;background:var(--surface-2);
  border-top:1px solid var(--border);max-height:180px;overflow-y:auto}
.cli-suggest.show{display:block}
.cli-sug{padding:6px 14px;cursor:pointer;font-size:11px;color:var(--text);
  display:flex;gap:8px;align-items:center;border-left:2px solid transparent}
.cli-sug:hover,.cli-sug.sel{background:var(--surface-3);color:var(--cyan);border-left-color:var(--cyan)}
.cli-sug .sug-prefix{color:var(--text-dim);font-size:10px}
.cli-sug .sug-arrow{margin-left:auto;color:var(--text-dim);font-size:10px}
.cli-shortcuts{display:flex;flex-wrap:wrap;gap:6px;margin-top:6px}
.cli-chip{font-size:10px;padding:3px 8px;border:1px solid var(--border);border-radius:3px;
  background:var(--surface-2);color:var(--text-dim);cursor:pointer;transition:all .12s;
  letter-spacing:.5px}
.cli-chip:hover{border-color:var(--magenta);color:var(--magenta)}

#sensitive-modal .modal{max-width:560px;border-color:var(--yellow)}
#sensitive-modal.danger .modal{border-color:var(--red);box-shadow:0 20px 60px rgba(255,23,68,.25)}
#sens-title{color:var(--yellow)}
#sensitive-modal.danger #sens-title{color:var(--red)}
#sens-body p{margin-bottom:6px}
#sens-body ul{list-style:none;padding:0;margin:8px 0;font-size:11px;line-height:1.8}
#sens-body .ok-item{color:var(--green)}
#sens-body .bad-item{color:var(--red)}
#sensitive-modal .actions{flex-wrap:wrap;margin-top:16px}
kbd{background:var(--surface-2);border:1px solid var(--border);border-radius:3px;
  padding:1px 5px;font-size:10px;color:var(--text);font-family:var(--mono)}
</style>
</head>
<body>

<div id="login" class="login">
  <div class="login-card">
    <div class="login-logo" id="login-logo"></div>
    <div class="login-brand">AXIS</div>
    <div class="login-sub">v6.0 · windows · acesso controlado</div>
    <div class="login-boot" id="login-boot"></div>
    <form id="login-form" hidden>
      <div class="login-field"><label>ID do Operador</label>
        <input type="text" id="login-user" autocomplete="off" required></div>
      <div class="login-field"><label>Código de Acesso</label>
        <input type="password" id="login-pass" autocomplete="off" required></div>
      <button type="submit" class="login-btn">AUTENTICAR</button>
      <div class="login-meta"><span>canal seguro</span><span>·</span><span>sessão efêmera</span></div>
    </form>
  </div>
  <div class="login-footer">build 6.0.0 · sessão local · política de escopo aplicada no servidor</div>
</div>

<div id="app" class="app" hidden>
  <header>
    <div class="brand"><span id="header-logo"></span><span class="brand-name">AXIS</span></div>
    <div class="status">
      <span class="pill" id="pill-scope"><span class="dot"></span><span class="label">escopo</span><span class="value">—</span></span>
      <span class="pill" id="pill-findings"><span class="dot"></span><span class="label">achados</span><span class="value">0</span></span>
      <span class="pill" id="pill-sentinel" title="SENTINEL engine"><span class="dot"></span><span class="label">sentinel</span><span class="value">—</span></span>
      <span class="pill" id="pill-host"><span class="dot"></span><span class="label">host</span><span class="value">—</span></span>
    </div>
    <div class="header-spacer"></div>
    <button class="hdr-btn" id="cli-btn">⌨ cli</button>
    <button class="hdr-btn" id="apis-btn">⚡ apis</button>
    <button class="hdr-btn" id="config-btn">⚙ sistema</button>
    <button class="hdr-btn" id="docs-btn">docs</button>
    <button class="hdr-btn danger" id="logout-btn">sair</button>
  </header>

  <aside class="sidebar">
    <div class="search-wrap">
      <input class="search" id="search" placeholder="filtrar módulos…  Ctrl+K" autocomplete="off">
    </div>
    <div id="recents" class="recents"></div>
    <div class="cat-list" id="cat-list"></div>
  </aside>

  <main class="main" id="main-app">
    <div class="pane">
      <h2 id="mod-title">selecione um módulo</h2>
      <div class="form-grid" id="form-grid"></div>
      <div class="actions" id="mod-actions"></div>
    </div>
    <div class="output">
      <div class="output-header">
        <div class="out-tabs">
          <button class="out-tab active" data-tab="stdout">stdout</button>
          <button class="out-tab" data-tab="stderr" data-empty="1">stderr</button>
          <button class="out-tab" data-tab="raw">raw</button>
        </div>
        <div class="out-actions">
          <span id="out-stat">—</span>
          <button class="out-act" id="out-copy">copiar</button>
          <button class="out-act" id="out-dl">salvar</button>
          <button class="out-act" id="out-clear">limpar</button>
        </div>
      </div>
      <pre id="out-pre">Aguardando seleção de módulo.</pre>
    </div>
  </main>

  <main class="main cli-view" id="main-cli" hidden>
    <div class="cli-wrap">
      <div class="cli-head">
        <div>
          <div class="cli-h1">⌨ Console AURA</div>
          <div class="cli-lead">digite <code style="color:var(--cyan)">help</code> ou <code style="color:var(--magenta)">aura</code>.</div>
        </div>
        <div><span style="font-size:10px;color:var(--text-dim)" id="cli-theme-label">theme: axis</span></div>
      </div>
      <div class="cli-console">
        <div class="cli-out" id="cli-out"></div>
        <div class="cli-suggest" id="cli-suggest"></div>
        <div class="cli-in-wrap">
          <span class="cli-prompt">❯</span>
          <input class="cli-input" id="cli-in" autocomplete="off" placeholder="aura · help · set · get · scope · audit · run">
        </div>
      </div>
      <div class="cli-shortcuts" id="cli-shortcuts">
        <span class="cli-chip" data-cmd="aura">✦ aura</span>
        <span class="cli-chip" data-cmd="aura vision">✦ aura vision</span>
        <span class="cli-chip" data-cmd="aura whoami">✦ aura whoami</span>
        <span class="cli-chip" data-cmd="aura themes">✦ aura themes</span>
        <span class="cli-chip" data-cmd="aura ritual">✦ aura ritual</span>
        <span class="cli-chip" data-cmd="audit">audit</span>
        <span class="cli-chip" data-cmd="keys">keys</span>
        <span class="cli-chip" data-cmd="scope">scope</span>
        <span class="cli-chip" data-cmd="ps">ps</span>
        <span class="cli-chip" data-cmd="clear">clear</span>
      </div>
    </div>
  </main>

  <main class="main apis-view" id="main-apis" hidden>
    <div class="output" style="padding:24px 28px">
      <div class="config-wrap">
        <h1 class="config-h1">⚡ APIs & Webhooks</h1>
        <p class="config-lead">Vai em <code>%USERPROFILE%\.axis\env</code> e é injetado em cada <code>axis</code>.</p>
        <div class="config-section">
          <h2>Reconhecimento externo</h2>
          <div class="apis-grid">
            <div class="apis-card" data-key="HIBP_KEY">
              <h3>HIBP<span class="bind-status" data-for="HIBP_KEY">—</span></h3>
              <div class="apis-hint">Have I Been Pwned.</div>
              <input type="password" data-for="HIBP_KEY" placeholder="hibp api key" autocomplete="off">
              <div class="apis-actions"><button class="action primary" data-act="bind" data-key="HIBP_KEY">BIND</button></div>
            </div>
            <div class="apis-card" data-key="SHODAN_KEY">
              <h3>Shodan<span class="bind-status" data-for="SHODAN_KEY">—</span></h3>
              <div class="apis-hint">banner search.</div>
              <input type="password" data-for="SHODAN_KEY" placeholder="shodan api key" autocomplete="off">
              <div class="apis-actions"><button class="action primary" data-act="bind" data-key="SHODAN_KEY">BIND</button></div>
            </div>
            <div class="apis-card" data-key="VT_KEY">
              <h3>VirusTotal<span class="bind-status" data-for="VT_KEY">—</span></h3>
              <div class="apis-hint">hash / arquivo.</div>
              <input type="password" data-for="VT_KEY" placeholder="virustotal key" autocomplete="off">
              <div class="apis-actions"><button class="action primary" data-act="bind" data-key="VT_KEY">BIND</button></div>
            </div>
            <div class="apis-card" data-key="GITHUB_TOKEN">
              <h3>GitHub<span class="bind-status" data-for="GITHUB_TOKEN">—</span></h3>
              <div class="apis-hint">PAT · supply chain.</div>
              <input type="password" data-for="GITHUB_TOKEN" placeholder="ghp_..." autocomplete="off">
              <div class="apis-actions"><button class="action primary" data-act="bind" data-key="GITHUB_TOKEN">BIND</button></div>
            </div>
          </div>
        </div>
        <div class="config-section">
          <h2>Canais de exfiltração</h2>
          <div class="apis-grid">
            <div class="apis-card" data-key="AXIS_STEALER_TG_TOKEN">
              <h3>Telegram · Token<span class="bind-status" data-for="AXIS_STEALER_TG_TOKEN">—</span></h3>
              <div class="apis-hint">bot token @BotFather.</div>
              <input type="password" data-for="AXIS_STEALER_TG_TOKEN" autocomplete="off">
              <div class="apis-actions"><button class="action primary" data-act="bind" data-key="AXIS_STEALER_TG_TOKEN">BIND</button></div>
            </div>
            <div class="apis-card" data-key="AXIS_STEALER_TG_CHAT">
              <h3>Telegram · Chat ID<span class="bind-status" data-for="AXIS_STEALER_TG_CHAT">—</span></h3>
              <div class="apis-hint">id do chat.</div>
              <input type="text" data-for="AXIS_STEALER_TG_CHAT" autocomplete="off">
              <div class="apis-actions"><button class="action primary" data-act="bind" data-key="AXIS_STEALER_TG_CHAT">BIND</button></div>
            </div>
            <div class="apis-card" data-key="AXIS_STEALER_DISCORD_WEBHOOK">
              <h3>Discord Webhook<span class="bind-status" data-for="AXIS_STEALER_DISCORD_WEBHOOK">—</span></h3>
              <div class="apis-hint">webhook completo.</div>
              <input type="password" data-for="AXIS_STEALER_DISCORD_WEBHOOK" autocomplete="off">
              <div class="apis-actions"><button class="action primary" data-act="bind" data-key="AXIS_STEALER_DISCORD_WEBHOOK">BIND</button></div>
            </div>
            <div class="apis-card" data-key="AXIS_CF_WORKER">
              <h3>Cloudflare Worker<span class="bind-status" data-for="AXIS_CF_WORKER">—</span></h3>
              <div class="apis-hint">domain fronting.</div>
              <input type="text" data-for="AXIS_CF_WORKER" autocomplete="off">
              <div class="apis-actions"><button class="action primary" data-act="bind" data-key="AXIS_CF_WORKER">BIND</button></div>
            </div>
          </div>
        </div>
        <div class="config-section">
          <h2>Proxy padrão</h2>
          <div class="apis-grid">
            <div class="apis-card" data-key="AXIS_PROXY_DEFAULT">
              <h3>Proxy<span class="bind-status" data-for="AXIS_PROXY_DEFAULT">—</span></h3>
              <div class="apis-hint">aplicado quando AXIS_PROXY_&lt;PROFILE&gt; não existe.</div>
              <input type="text" data-for="AXIS_PROXY_DEFAULT" placeholder="socks5://127.0.0.1:1080" autocomplete="off">
              <div class="apis-actions"><button class="action primary" data-act="bind" data-key="AXIS_PROXY_DEFAULT">BIND</button></div>
            </div>
          </div>
        </div>
        <div class="config-actions">
          <button class="action" onclick="apisReload()">RECARREGAR</button>
          <span id="apis-status" style="margin-left:12px;color:var(--text-dim);font-size:11px"></span>
        </div>
      </div>
    </div>
  </main>

  <main class="main config-view" id="main-config" hidden>
    <div class="output" style="padding:24px 28px">
      <div class="config-wrap">
        <h1 class="config-h1">⚙ Sistema</h1>
        <p class="config-lead">Porta, binário, processos, escopo, OPSEC, SENTINEL engine.</p>

        <div class="cfg-status" id="cfg-status">
          <span>OPSEC:</span>
          <span class="cfg-pill" id="cfg-status-scope">—</span>
          <span class="cfg-pill" id="cfg-status-vpn">—</span>
          <span class="cfg-pill" id="cfg-status-mac">—</span>
          <button class="action" onclick="configRefreshOpsec()" style="margin-left:auto;padding:3px 10px;font-size:10px">REFRESH</button>
        </div>

        <div class="config-section" style="margin-top:20px">
          <h2>Servidor</h2>
          <div class="config-fields">
            <div class="config-field"><label>Porta do GUI</label><input type="text" id="cfg-port" readonly></div>
            <div class="config-field"><label>Binário axis</label><input type="text" id="cfg-bin" readonly></div>
            <div class="config-field"><label>Processos em background</label><div class="cfg-procs" id="cfg-procs">—</div></div>
          </div>
          <div class="config-actions">
            <button class="action" onclick="configServerTest()">TESTAR BINÁRIO</button>
            <button class="action" onclick="configSpawnServer('track', ['track'])">INICIAR TRACK</button>
            <button class="action" onclick="configSpawnServer('c2', ['c2'])">INICIAR C2</button>
            <button class="action" onclick="configSpawnServer('phish', ['phish','--port','8080'])">INICIAR PHISH</button>
            <button class="action danger" onclick="configKillAll()">PARAR TODOS</button>
          </div>
        </div>

        <div class="config-section">
          <h2>SENTINEL · engine</h2>
          <p class="config-p">O engine (<code>sentinel_engine.py</code>) roda em <code>127.0.0.1:8899</code>. Os módulos da
            categoria <b>SENTINEL</b> só funcionam com ele no ar. A pílula no header mostra
            <span style="color:var(--green)">verde</span> = online,
            <span style="color:var(--yellow)">amarelo</span> = incidente,
            <span style="color:var(--red)">vermelho</span> = offline.</p>
          <div class="config-actions">
            <button class="action" onclick="sentinelHealth()">TESTAR ENGINE</button>
            <button class="action" onclick="sentinelStart()">SENTINEL · START</button>
            <button class="action danger" onclick="sentinelStop()">SENTINEL · STOP</button>
          </div>
          <div class="spacer" style="height:8px"></div>
          <div class="pre" id="sentinel-status" style="background:var(--surface-2);border:1px solid var(--border);padding:10px;font-size:11px;color:var(--text-dim);white-space:pre-wrap">—</div>
        </div>

        <div class="config-section">
          <h2>Escopo (scope.yaml)</h2>
          <textarea id="cfg-scope" class="config-scope"></textarea>
          <div class="config-actions">
            <button class="action primary" onclick="configSaveScope()">SALVAR ESCOPO</button>
            <button class="action" onclick="configReload()">RECARREGAR</button>
            <span id="cfg-save-status" style="margin-left:12px;color:var(--text-dim);font-size:11px"></span>
          </div>
        </div>

        <div class="config-section">
          <h2>⚡ OPSEC automático</h2>
          <div class="config-actions">
            <button class="action" onclick="configRun(['opsec','audit'], {name:'Auditoria OPSEC'})">AUDITAR AGORA</button>
            <button class="action danger" onclick="configRun(['opsec','killswitch','enable','--iface','WireGuard'], {name:'Ativar killswitch', irreversible:true})">ATIVAR KILLSWITCH</button>
            <button class="action danger" onclick="configRun(['opsec','mac','set','Wi-Fi'], {name:'Rotacionar MAC', irreversible:true})">ROTACIONAR MAC</button>
          </div>
        </div>
      </div>
    </div>
  </main>

  <main class="main docs-view" id="main-docs" hidden>
    <div class="output" style="padding:24px 28px">
      <div class="docs-wrap">
        <h1 class="docs-h1">AXIS v6.0 · Windows · como usar</h1>
        <p class="docs-lead">Plataforma unificada para avaliação de segurança, emulação de adversário e defesa ativa.
          Cada clique mapeia para <code>axis &lt;subcomando&gt;</code> ou para o engine em <code>127.0.0.1:8899</code>.</p>

        <h2 class="docs-h2">▸ abas</h2>
        <ul class="docs-ul">
          <li><b>módulos</b> — sidebar + form + output. Trabalho principal.</li>
          <li><b>⌨ cli</b> — terminal com comandos <code>aura</code>.</li>
          <li><b>⚡ apis</b> — chaves de API + webhooks + proxy.</li>
          <li><b>⚙ sistema</b> — porta, binário, processos, SENTINEL engine, escopo, OPSEC.</li>
          <li><b>docs</b> — este arquivo.</li>
        </ul>

        <h2 class="docs-h2">▸ categorias</h2>
        <div class="docs-cats" id="docs-cats"></div>

        <h2 class="docs-h2">▸ expansões v6.0</h2>
        <ul class="docs-ul">
          <li><b>Emulação Avançada</b> — stack spoofing (Moonwalk++), resolução de syscall
            (Tartarus Gate), suspensão de EDR em lab, Hyper-V NAT, teste BYOVD, ADCS ESC, IPI/LLM.</li>
          <li><b>Detecção & Resposta</b> — Sysmon + ETW, sentinel de log, caça-injeção,
            guarda do dumper, WDAC (audit/enforce), olho de hipervisor, playbook de contenção.</li>
          <li><b>SENTINEL · Defesa Ativa</b> — engine modular com 5 monitores paralelos:
            registry (Run/RunOnce/ms-settings), processos (PowerShell oculto, hook de teclado),
            rede (netstat → domínios suspeitos), tarefas agendadas, arquivos (YARA + heurística).
            Correlator promove ≥2 fontes distintas na mesma janela de 60s para <b>incident</b>
            e envia um único webhook.</li>
          <li>Módulos com <span style="color:var(--red)">●</span> passam pelo modal <code>EXECUTE</code>
            e são re-verificados no servidor via <code>is_irreversible</code>.</li>
        </ul>

        <h2 class="docs-h2">▸ SENTINEL · como subir o engine</h2>
        <pre class="docs-pre"># terminal à parte, com o venv ativo
pip install psutil yara-python requests
python sentinel_engine.py --port 8899

# verificar
curl http://127.0.0.1:8899/health</pre>
        <p class="docs-p">A GUI detecta o engine sozinha (pílula <b>sentinel</b> no header).
          Sem o engine, os módulos SENTINEL retornam <code>sentinel engine offline em 127.0.0.1:8899</code>
          — visível no output, não silencioso.</p>

        <h2 class="docs-h2">▸ operações sensíveis (Windows)</h2>
        <ul class="docs-ul">
          <li><code>opsec mac set|reset</code> · <code>opsec killswitch enable|disable</code> · <code>opsec persist install</code></li>
          <li><code>forensics disk|memory</code> · <code>persist *</code></li>
          <li><code>defender disable|exclusions</code> · <code>firewall disable</code> · <code>bitlocker off</code></li>
          <li><code>bcdedit</code> · <code>vssadmin</code> · <code>wevtutil cl</code> · <code>schtasks delete</code> · <code>reg delete</code> · <code>sc delete</code></li>
          <li><code>hardening --action asr-enable|credguard-enable|smb-harden|firewall-default-deny|defender-aggressive|wdac-enforce</code></li>
          <li><code>evasion freeze-edr|hyperv-nat</code> · <code>emulate byovd-killer</code> · <code>defense --action sysmon-install|wdac-enforce|contain</code></li>
        </ul>

        <h2 class="docs-h2">▸ uso autorizado</h2>
        <p class="docs-p">Laboratórios próprios, sistemas com autorização escrita, engajamentos
          contratados. <code>AUTHORIZED_USE.md</code> é a declaração vinculante.</p>
      </div>
    </div>
  </main>

  <footer>
    <span>127.0.0.1:7878</span><span>·</span>
    <span id="footer-cmd">pronto</span>
    <span class="footer-spacer"></span>
    <span id="footer-operator">operador: —</span>
    <span>·</span>
    <span>AXIS v6.0</span>
  </footer>
</div>

<div class="modal-backdrop" id="sensitive-modal">
  <div class="modal">
    <h3 id="sens-title">⚠ Ação Sensível</h3>
    <div id="sens-body"></div>
    <div id="sens-irrev-section" hidden>
      <p style="font-size:11px;color:var(--text-dim)">digite <b style="color:var(--yellow)">EXECUTE</b>:</p>
      <input type="text" id="sens-confirm-input" class="confirm-input" autocomplete="off" placeholder="EXECUTE">
    </div>
    <div class="actions">
      <button class="action" id="sens-cancel">CANCELAR</button>
      <button class="action" id="sens-config" style="display:none">CONFIGURAR OPSEC</button>
      <button class="action" id="sens-audit" style="display:none">RODAR AUDIT</button>
      <button class="action danger" id="sens-go" disabled>EXECUTAR</button>
    </div>
  </div>
</div>

<div id="toasts" class="toasts"></div>

<div class="palette-backdrop" id="palette">
  <div class="palette">
    <input class="palette-input" id="palette-input" placeholder="pular para módulo…" autocomplete="off">
    <div class="palette-list" id="palette-list"></div>
  </div>
</div>

<script>
// ═════════ THEMES ═════════
const THEMES = {
  axis:   { cyan:'#00e5ff', magenta:'#ff00e5', green:'#00e676' },
  aurora: { cyan:'#22d3ee', magenta:'#a855f7', green:'#22c55e' },
  matrix: { cyan:'#22c55e', magenta:'#16a34a', green:'#4ade80' },
  blood:  { cyan:'#ef4444', magenta:'#dc2626', green:'#22c55e' },
  ghost:  { cyan:'#94a3b8', magenta:'#cbd5e1', green:'#10b981' },
};
function applyTheme(name){
  const t = THEMES[name] || THEMES.axis;
  const r = document.documentElement.style;
  r.setProperty('--cyan', t.cyan);
  r.setProperty('--magenta', t.magenta);
  r.setProperty('--green', t.green);
  localStorage.setItem('axis_theme', name);
  const lbl = document.getElementById('cli-theme-label');
  if (lbl) lbl.textContent = 'theme: ' + name;
}

// ═════════ LOGO / BOOT ═════════
function makeLogo(size, glow=false){
  const id = 'lg' + size + (glow ? 'g' : '');
  const style = glow ? 'filter:drop-shadow(0 0 10px rgba(0,229,255,.55))' : '';
  return `<svg viewBox="0 0 64 64" width="${size}" height="${size}" xmlns="http://www.w3.org/2000/svg" style="${style}">
    <defs><linearGradient id="${id}" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0" stop-color="#00e5ff"/><stop offset="1" stop-color="#ff00e5"/>
    </linearGradient></defs>
    <polygon points="32,4 56,18 56,46 32,60 8,46 8,18" fill="none" stroke="url(#${id})" stroke-width="1.6" opacity="0.75"/>
    <polygon points="32,12 47,21 47,43 32,52 17,43 17,21" fill="none" stroke="url(#${id})" stroke-width="1" opacity="0.35"/>
    <path d="M22 44 L32 20 L42 44 M25 36 L39 36" fill="none" stroke="url(#${id})" stroke-width="2.6" stroke-linecap="round"/>
  </svg>`;
}
const BOOT_LINES = [
  '> INICIANDO CANAL SEGURO',
  '> HANDSHAKE TLS 1.3 <span class="ok">[OK]</span>',
  '> POLÍTICA DE ESCOPO ARMADA',
  '> AGUARDANDO CREDENCIAIS <span class="ok">_</span>',
];
const sleep = ms => new Promise(r => setTimeout(r, ms));
const appStart = Date.now();

async function bootLogin(){
  document.getElementById('login-logo').innerHTML = makeLogo(64, true);
  const boot = document.getElementById('login-boot');
  const form = document.getElementById('login-form');
  boot.innerHTML = '';
  for (const line of BOOT_LINES){
    boot.innerHTML += line + '<span class="boot-cursor">▊</span>\n';
    await sleep(180);
  }
  await sleep(150);
  form.hidden = false;
  document.getElementById('login-user').focus();
}
document.getElementById('login-form').onsubmit = e => {
  e.preventDefault();
  const u = document.getElementById('login-user').value.trim();
  const p = document.getElementById('login-pass').value;
  if (!u || !p) return;
  localStorage.setItem('axis_auth','1');
  localStorage.setItem('axis_operator', u);
  const btn = e.target.querySelector('.login-btn');
  btn.textContent = '▸ VERIFICANDO';
  btn.style.pointerEvents = 'none';
  setTimeout(showApp, 380);
};
function showApp(){
  applyTheme(localStorage.getItem('axis_theme') || 'axis');
  const login = document.getElementById('login');
  const app = document.getElementById('app');
  login.classList.add('leaving');
  app.hidden = false;
  app.classList.add('entering');
  document.getElementById('header-logo').innerHTML = makeLogo(22);
  const op = localStorage.getItem('axis_operator') || 'anon';
  document.getElementById('footer-operator').textContent = 'operador: ' + op;
  renderSidebar('');
  renderRecents();
  refreshState();
  setInterval(refreshState, 5000);
  setInterval(tickUptime, 1000);
  const first = MODULES.find(m => m.cat === 'ascii') || MODULES[0];
  if (first) selectModule(first);
  setTimeout(() => { login.style.display = 'none'; }, 700);
}
document.getElementById('logout-btn').onclick = () => {
  localStorage.removeItem('axis_auth');
  localStorage.removeItem('axis_operator');
  location.reload();
};

// ═════════ CATEGORIES ═════════
const CATEGORIES = [
  { id:'ascii',        name:'Branding & Utilidades', color:'#00e5ff' },
  { id:'opsec',        name:'OPSEC · Operador',     color:'#ffc400' },
  { id:'recon',        name:'Reconhecimento',       color:'#00bcd4' },
  { id:'osint',        name:'OSINT · Inteligência Pública', color:'#b388ff' },
  { id:'attack',       name:'Emulação de Adversário', color:'#ff1744' },
  { id:'credential',   name:'Credential Hygiene',   color:'#ff5252' },
  { id:'defense',      name:'Defesa Ativa',         color:'#00e676' },
  { id:'purple',       name:'Purple Team',          color:'#e040fb' },
  { id:'hardening',    name:'Hardening · Endurecimento', color:'#76ff03' },
  { id:'redteam_adv',  name:'Emulação Avançada',    color:'#ff0033' },
  { id:'blueteam_adv', name:'Detecção & Resposta',  color:'#00e5a0' },
  { id:'hades',        name:'SENTINEL · Defesa Ativa', color:'#a855f7' },
  { id:'cloud',        name:'Cloud Security',       color:'#40c4ff' },
  { id:'container',    name:'Container Security',   color:'#1de9b6' },
  { id:'iot',          name:'IoT · Embarcados',     color:'#ff9100' },
  { id:'hardware',     name:'Hardware',             color:'#ffd740' },
  { id:'supply_chain', name:'Supply Chain',         color:'#7c4dff' },
  { id:'forensics',    name:'Forense Digital',      color:'#90a4ae' },
  { id:'reporting',    name:'Relatórios',           color:'#e0e0e0' },
];
const CAT_MAP = Object.fromEntries(CATEGORIES.map(c => [c.id, c]));
const catColor = id => (CAT_MAP[id] || {}).color || '#888';
const catName  = id => (CAT_MAP[id] || {}).name  || id;

// ═════════ MODULES ═════════
const MODULES = [
  { id:'ascii-banner', cat:'ascii', name:'Banner', args:[
    { name:'text', label:'Texto', type:'text', default:'AXIS v6.0', required:true },
    { name:'font', label:'Fonte', type:'select', options:['slant','standard','big'], default:'slant' },
    { name:'gradient', label:'Gradiente', type:'text', default:'cyan-magenta' }
  ], build: v => ['ascii','banner', v.text, '--font', v.font, '--gradient', v.gradient] },
  { id:'ascii-img', cat:'ascii', name:'Imagem → ASCII', args:[
    { name:'path', label:'Caminho', type:'text', required:true },
    { name:'width', label:'Largura (px)', type:'number', default:120 }
  ], build: v => ['ascii','img', v.path, '--width', String(v.width)] },

  { id:'opsec-audit', cat:'opsec', name:'Auditoria', args:[], build: () => ['opsec','audit'] },
  { id:'opsec-mac-status', cat:'opsec', name:'MAC · Status', args:[
    { name:'iface', label:'Interface', type:'text', default:'Wi-Fi' }
  ], build: v => ['opsec','mac','status', v.iface] },
  { id:'opsec-mac-set', cat:'opsec', name:'MAC · Configurar', irreversible:true, args:[
    { name:'iface', label:'Interface', type:'text', default:'Wi-Fi' },
    { name:'mac', label:'Novo MAC (vazio=rand)', type:'text' }
  ], build: v => v.mac ? ['opsec','mac','set', v.iface, '--mac', v.mac] : ['opsec','mac','set', v.iface] },
  { id:'opsec-ks-status', cat:'opsec', name:'Killswitch · Status', args:[
    { name:'iface', label:'Interface VPN', type:'text', default:'WireGuard' }
  ], build: v => ['opsec','killswitch','--iface', v.iface] },
  { id:'opsec-ks-enable', cat:'opsec', name:'Killswitch · Ativar', irreversible:true, args:[
    { name:'iface', label:'Interface VPN', type:'text', default:'WireGuard' }
  ], build: v => ['opsec','killswitch','enable','--iface', v.iface] },
  { id:'opsec-ks-disable', cat:'opsec', name:'Killswitch · Desativar', irreversible:true, args:[], build: () => ['opsec','killswitch','disable'] },
  { id:'opsec-leak-stun', cat:'opsec', name:'Vazamento · STUN', args:[], build: () => ['opsec','leak','stun'] },
  { id:'opsec-tor-rotate', cat:'opsec', name:'Tor · NEWNYM', args:[], build: () => ['opsec','tor','rotate'] },

  { id:'recon-scan', cat:'recon', name:'Varredura de Portas', args:[
    { name:'target', label:'Alvo', type:'text', required:true },
    { name:'ports', label:'Portas', type:'text', default:'1-1000' }
  ], build: v => ['recon','--action','scan','--target', v.target, '--ports', v.ports] },
  { id:'recon-banner', cat:'recon', name:'Captura de Banner', args:[
    { name:'target', label:'Alvo', type:'text', required:true },
    { name:'port', label:'Porta', type:'number', default:80 }
  ], build: v => ['recon','--action','banner','--target', v.target, '--port', String(v.port)] },
  { id:'recon-dns', cat:'recon', name:'Enumeração DNS', args:[
    { name:'target', label:'Domínio', type:'text', required:true }
  ], build: v => ['recon','--action','dns','--target', v.target] },
  { id:'recon-web', cat:'recon', name:'Fingerprint Web', args:[
    { name:'target', label:'URL', type:'text', required:true }
  ], build: v => ['recon','--action','web','--target', v.target] },

  { id:'osint-dork', cat:'osint', name:'Gerador de Dorks', args:[
    { name:'domain', label:'Domínio', type:'text', required:true }
  ], build: v => ['osint','--action','dork','--domain', v.domain] },
  { id:'osint-whois', cat:'osint', name:'WHOIS', args:[
    { name:'target', label:'Domínio / IP', type:'text', required:true }
  ], build: v => ['osint','--action','whois','--target', v.target] },
  { id:'osint-hibp', cat:'osint', name:'Consulta HIBP', args:[
    { name:'email', label:'Email', type:'text', required:true }
  ], build: v => ['osint','--action','hibp','--email', v.email] },
  { id:'osint-shodan', cat:'osint', name:'Busca Shodan', args:[
    { name:'q', label:'Consulta', type:'text', required:true }
  ], build: v => ['osint','--action','shodan','--q', v.q] },

  { id:'atk-pw-gen', cat:'attack', name:'Wordlist · Geração', args:[
    { name:'base', label:'Palavra base', type:'text', required:true },
    { name:'target', label:'Empresa', type:'text' }
  ], build: v => v.target
    ? ['password','--action','gen','--base', v.base, '--target', v.target]
    : ['password','--action','gen','--base', v.base] },
  { id:'atk-wifi-scan', cat:'attack', name:'Wireless · Descoberta', args:[
    { name:'iface', label:'Interface', type:'text', default:'Wi-Fi' }
  ], build: v => ['wireless','--action','scan','--iface', v.iface] },
  { id:'atk-wifi-pmkid', cat:'attack', name:'Wireless · Handshake', args:[
    { name:'iface', label:'Interface', type:'text', default:'Wi-Fi' },
    { name:'bssid', label:'BSSID', type:'text', required:true }
  ], build: v => ['wireless','--action','pmkid','--iface', v.iface, '--bssid', v.bssid] },
  { id:'atk-privesc-enum', cat:'attack', name:'Privesc · Enumerar', args:[], build: () => ['privesc','--action','enum'] },
  { id:'atk-persist', cat:'attack', name:'Persistência · Agendador', irreversible:true, args:[
    { name:'binary', label:'Binário', type:'text', required:true },
    { name:'target', label:'Alvo', type:'text', required:true }
  ], build: v => ['persist','--method','schtasks','--binary', v.binary, '--target', v.target] },
  { id:'atk-ad-enum', cat:'attack', name:'Directory · Enumeração', args:[
    { name:'dc', label:'DC', type:'text', required:true }
  ], build: v => ['directory','--action','enum','--dc', v.dc] },
  { id:'atk-ad-kerb', cat:'attack', name:'Directory · Auditoria Kerberos', args:[
    { name:'dc', label:'DC', type:'text', required:true }
  ], build: v => ['directory','--action','kerberoast','--dc', v.dc] },

  { id:'cred-browsers', cat:'credential', name:'Navegadores · Inventário', args:[
    { name:'browser', label:'Navegador', type:'select', options:['all','chrome','edge','firefox','brave'], default:'all' },
    { name:'out', label:'Saída ZIP', type:'text', default:'%TEMP%\\axis-browsers.zip' }
  ], build: v => ['collect','--action','browsers','--browser', v.browser, '--out', v.out] },
  { id:'cred-wallets', cat:'credential', name:'Carteiras Digitais', args:[], build: () => ['collect','--action','wallets'] },
  { id:'cred-apps', cat:'credential', name:'Aplicações · Inventário', args:[
    { name:'target', label:'Alvo', type:'select', options:['all','telegram','whatsapp','gaming'], default:'all' }
  ], build: v => ['collect','--action','apps','--target', v.target] },
  { id:'cred-rotate', cat:'credential', name:'Rotação de Sessão', args:[], build: () => ['collect','--action','rotate'] },
  { id:'cred-keepalive', cat:'credential', name:'Keepalive', args:[], build: () => ['collect','--action','keepalive'] },

  { id:'def-defender', cat:'defense', name:'Defender · Status', args:[], build: () => ['defense','--action','defender-status'] },
  { id:'def-firewall', cat:'defense', name:'Firewall · Status', args:[], build: () => ['defense','--action','firewall-status'] },
  { id:'def-autoruns', cat:'defense', name:'Autoruns', args:[], build: () => ['defense','--action','autoruns'] },
  { id:'def-schtasks', cat:'defense', name:'Tarefas agendadas', args:[], build: () => ['defense','--action','schtasks'] },
  { id:'def-services', cat:'defense', name:'Serviços', args:[], build: () => ['defense','--action','services'] },
  { id:'def-patch', cat:'defense', name:'Patch level', args:[], build: () => ['defense','--action','patch-level'] },
  { id:'def-bitlocker', cat:'defense', name:'BitLocker', args:[], build: () => ['defense','--action','bitlocker'] },
  { id:'def-credguard', cat:'defense', name:'Credential Guard', args:[], build: () => ['defense','--action','credguard'] },
  { id:'def-smb', cat:'defense', name:'SMB hardening', args:[], build: () => ['defense','--action','smb'] },
  { id:'def-yara', cat:'defense', name:'YARA scan', args:[
    { name:'path', label:'Caminho', type:'text', default:'C:\\' }
  ], build: v => ['defense','--action','yara','--path', v.path] },
  { id:'def-evtx', cat:'defense', name:'Event log query', args:[
    { name:'log', label:'Log', type:'select', options:['Security','System','Application','Microsoft-Windows-Sysmon/Operational'], default:'Security' },
    { name:'id', label:'Event ID', type:'text', default:'4624' }
  ], build: v => ['defense','--action','evtx','--log', v.log, '--id', v.id] },
  { id:'def-persistence', cat:'defense', name:'Persistence audit', args:[], build: () => ['defense','--action','persistence'] },
  { id:'def-lolbas', cat:'defense', name:'LOLBAS detect', args:[], build: () => ['defense','--action','lolbas'] },
  { id:'def-amsi', cat:'defense', name:'AMSI status', args:[], build: () => ['defense','--action','amsi'] },
  { id:'def-etw', cat:'defense', name:'ETW sessions', args:[], build: () => ['defense','--action','etw'] },
  { id:'def-wdac', cat:'defense', name:'WDAC / AppLocker', args:[], build: () => ['defense','--action','wdac'] },
  { id:'def-laps', cat:'defense', name:'LAPS status', args:[], build: () => ['defense','--action','laps'] },
  { id:'def-asr', cat:'defense', name:'ASR rules', args:[], build: () => ['defense','--action','asr'] },
  { id:'def-net', cat:'defense', name:'Conexões de rede', args:[], build: () => ['defense','--action','net-connections'] },
  { id:'def-procs', cat:'defense', name:'Processos assinados', args:[], build: () => ['defense','--action','signed-procs'] },

  { id:'pur-atomic', cat:'purple', name:'Atomic Red Team', args:[
    { name:'technique', label:'Técnica', type:'text', default:'T1059.001' }
  ], build: v => ['purple','--action','atomic','--technique', v.technique] },
  { id:'pur-coverage', cat:'purple', name:'Coverage map', args:[], build: () => ['purple','--action','coverage'] },
  { id:'pur-caldera', cat:'purple', name:'Caldera operation', args:[], build: () => ['purple','--action','caldera'] },

  { id:'hard-baseline', cat:'hardening', name:'CIS baseline', args:[], build: () => ['hardening','--action','cis'] },
  { id:'hard-asr', cat:'hardening', name:'ASR enable', irreversible:true, args:[], build: () => ['hardening','--action','asr-enable'] },
  { id:'hard-credguard', cat:'hardening', name:'Credential Guard enable', irreversible:true, args:[], build: () => ['hardening','--action','credguard-enable'] },
  { id:'hard-smb', cat:'hardening', name:'SMB hardening', irreversible:true, args:[], build: () => ['hardening','--action','smb-harden'] },
  { id:'hard-firewall', cat:'hardening', name:'Firewall deny-by-default', irreversible:true, args:[], build: () => ['hardening','--action','firewall-default-deny'] },
  { id:'hard-defender', cat:'hardening', name:'Defender aggressive', irreversible:true, args:[], build: () => ['hardening','--action','defender-aggressive'] },

  // ── EMULAÇÃO AVANÇADA ──
  { id:'rt-moonwalk', cat:'redteam_adv', name:'Carregamento Seguro (Moonwalk++)', args:[
    { name:'pid', label:'PID alvo', type:'number', required:true },
    { name:'payload', label:'Payload (hex)', type:'text', required:true }
  ], build: v => ['evasion','stack-spoof','--pid', String(v.pid), '--payload', v.payload] },
  { id:'rt-tartarus', cat:'redteam_adv', name:'Resolução de Syscall (Tartarus)', args:[
    { name:'dll', label:'DLL', type:'select', options:['ntdll','kernelbase','win32u'], default:'ntdll' },
    { name:'func', label:'Função', type:'text', default:'NtAllocateVirtualMemory', required:true }
  ], build: v => ['evasion','syscall-resolve','--dll', v.dll, '--func', v.func] },
  { id:'rt-edr-freeze', cat:'redteam_adv', name:'Suspensão EDR (lab)', irreversible:true, args:[
    { name:'proc', label:'Processo EDR', type:'text', default:'MsMpEng.exe', required:true }
  ], build: v => ['evasion','freeze-edr','--proc', v.proc] },
  { id:'rt-hyperv', cat:'redteam_adv', name:'Isolamento Hyper-V NAT', irreversible:true, args:[
    { name:'image', label:'Imagem VM', type:'text', required:true },
    { name:'iface', label:'Interface NAT', type:'text', default:'vEthernet (External)' }
  ], build: v => ['evasion','hyperv-nat','--image', v.image, '--iface', v.iface] },
  { id:'rt-byovd', cat:'redteam_adv', name:'BYOVD · Teste de Kernel', irreversible:true, args:[
    { name:'driver', label:'Driver (.sys)', type:'text', required:true },
    { name:'proc', label:'Processo alvo', type:'text', default:'MsMpEng.exe', required:true }
  ], build: v => ['emulate','byovd-killer','--driver', v.driver, '--proc', v.proc] },
  { id:'rt-adcs', cat:'redteam_adv', name:'ADCS · Avaliação', args:[
    { name:'dc', label:'DC', type:'text', required:true },
    { name:'esc', label:'ESC', type:'select', options:['1','4','8'], default:'1' }
  ], build: v => ['directory','--action','adcs','--dc', v.dc, '--esc', v.esc] },
  { id:'rt-ipi', cat:'redteam_adv', name:'IPI/LLM · Auditoria', args:[
    { name:'target', label:'Alvo URL', type:'text', required:true },
    { name:'mode', label:'Modo', type:'select', options:['browser','email','rag'], default:'browser' }
  ], build: v => ['llm','--action','ipi','--target', v.target, '--mode', v.mode] },

  // ── DETECÇÃO & RESPOSTA ──
  { id:'bt-sysmon-status', cat:'blueteam_adv', name:'Sysmon · Status', args:[], build: () => ['defense','--action','sysmon-status'] },
  { id:'bt-sysmon-install', cat:'blueteam_adv', name:'Sysmon · Instalar', irreversible:true, args:[
    { name:'config', label:'Config XML', type:'text', default:'sysmonconfig-export.xml' }
  ], build: v => ['defense','--action','sysmon-install','--config', v.config] },
  { id:'bt-log-sentinel', cat:'blueteam_adv', name:'Sentinel de Log', args:[], build: () => ['defense','--action','log-sentinel'] },
  { id:'bt-hunt-inj', cat:'blueteam_adv', name:'Caça-Injeção (mem scan)', args:[
    { name:'pid', label:'PID (vazio = todos)', type:'number' }
  ], build: v => v.pid ? ['defense','--action','hunt-injection','--pid', String(v.pid)] : ['defense','--action','hunt-injection'] },
  { id:'bt-guard-dumper', cat:'blueteam_adv', name:'Guarda do Dumper', args:[], build: () => ['defense','--action','guard-dumper'] },
  { id:'bt-wdac-audit', cat:'blueteam_adv', name:'WDAC · Audit', args:[], build: () => ['defense','--action','wdac-audit'] },
  { id:'bt-wdac-enforce', cat:'blueteam_adv', name:'WDAC · Enforce', irreversible:true, args:[], build: () => ['defense','--action','wdac-enforce'] },
  { id:'bt-hyperv-watch', cat:'blueteam_adv', name:'Olho de Hipervisor', args:[], build: () => ['defense','--action','hyperv-watch'] },
  { id:'bt-contain', cat:'blueteam_adv', name:'Playbook de Contenção', irreversible:true, args:[
    { name:'pid', label:'PID alvo', type:'number', required:true },
    { name:'do', label:'Ação', type:'select', options:['isolate','suspend','dump','ioc'], default:'isolate' }
  ], build: v => ['defense','--action','contain','--pid', String(v.pid), '--do', v.do] },

  // ── SENTINEL · DEFESA ATIVA ──
  { id:'sn-registry', cat:'hades', name:'Registry Monitor', args:[], build: () => ['defense','--action','sentinel-registry'] },
  { id:'sn-process', cat:'hades', name:'Process Monitor', args:[], build: () => ['defense','--action','sentinel-process'] },
  { id:'sn-network', cat:'hades', name:'Network Monitor', args:[
    { name:'interval', label:'Intervalo (s)', type:'number', default:20 }
  ], build: v => ['defense','--action','sentinel-network','--interval', String(v.interval)] },
  { id:'sn-tasks', cat:'hades', name:'Task Monitor', args:[], build: () => ['defense','--action','sentinel-tasks'] },
  { id:'sn-files', cat:'hades', name:'File Monitor (YARA)', args:[
    { name:'path', label:'Diretório', type:'text', default:'%USERPROFILE%' },
    { name:'rules', label:'Rules', type:'select', options:['default','custom'], default:'default' }
  ], build: v => ['defense','--action','sentinel-files','--path', v.path, '--rules', v.rules] },
  { id:'sn-start', cat:'hades', name:'▶ Iniciar monitores', args:[], build: () => ['defense','--action','sentinel-start'] },
  { id:'sn-stop', cat:'hades', name:'■ Parar monitores', args:[], build: () => ['defense','--action','sentinel-stop'] },
  { id:'sn-log', cat:'hades', name:'Ver log do SENTINEL', args:[
    { name:'tail', label:'Linhas', type:'number', default:100 }
  ], build: v => ['defense','--action','sentinel-log','--tail', String(v.tail)] },

  // ── CLOUD / CONTAINER / IOT / HARDWARE / SUPPLY / FORENSICS / REPORT ──
  { id:'cl-aws-enum', cat:'cloud', name:'AWS · Enumerar', args:[], build: () => ['cloud','--provider','aws','--action','enum'] },
  { id:'cl-aws-s3', cat:'cloud', name:'AWS · S3', args:[
    { name:'bucket', label:'Bucket', type:'text', required:true }
  ], build: v => ['cloud','--provider','aws','--action','s3','--bucket', v.bucket] },
  { id:'cl-aws-imds', cat:'cloud', name:'AWS · IMDS', args:[], build: () => ['cloud','--provider','aws','--action','imds'] },
  { id:'cl-az-devcode', cat:'cloud', name:'Azure · Device Code', args:[
    { name:'tenant', label:'Tenant', type:'text', required:true }
  ], build: v => ['cloud','--provider','azure','--action','devicecode','--tenant', v.tenant] },
  { id:'cl-gcp-imds', cat:'cloud', name:'GCP · IMDS', args:[], build: () => ['cloud','--provider','gcp','--action','imds'] },
  { id:'ct-docker', cat:'container', name:'Docker · Enumerar', args:[], build: () => ['container','--action','docker'] },
  { id:'ct-k8s', cat:'container', name:'K8s · RBAC', args:[
    { name:'api', label:'API URL', type:'text', default:'https://kubernetes.default.svc' }
  ], build: v => ['container','--action','k8s','--api', v.api] },
  { id:'iot-fw', cat:'iot', name:'Firmware', args:[
    { name:'path', label:'Imagem', type:'text', required:true }
  ], build: v => ['iot','--action','firmware','--path', v.path] },
  { id:'iot-mqtt', cat:'iot', name:'MQTT · Subscribe', args:[
    { name:'broker', label:'Broker', type:'text', default:'127.0.0.1:1883' },
    { name:'subscribe', label:'Tópico', type:'text', default:'#' },
    { name:'seconds', label:'Duração', type:'number', default:30 }
  ], build: v => ['iot','--action','mqtt','--broker', v.broker, '--subscribe', v.subscribe, '--seconds', String(v.seconds)] },
  { id:'hw-pm3', cat:'hardware', name:'Proxmark3', args:[
    { name:'cmd', label:'Comando', type:'text', default:'hw version' },
    { name:'port', label:'Porta COM', type:'text', default:'COM3' }
  ], build: v => ['hardware','--action','proxmark','--port', v.port, '--cmd', v.cmd] },
  { id:'sc-typo', cat:'supply_chain', name:'Typosquat', args:[
    { name:'name', label:'Pacote', type:'text', required:true }
  ], build: v => ['supply_chain','--action','typosquat','--name', v.name] },
  { id:'fs-triage', cat:'forensics', name:'Triagem IR', args:[], build: () => ['forensics','--action','triage'] },
  { id:'fs-mem', cat:'forensics', name:'Aquisição Memória', irreversible:true, args:[
    { name:'out', label:'Saída', type:'text', required:true, default:'%TEMP%\\axis-mem.raw' }
  ], build: v => ['forensics','--action','memory','--out', v.out] },
  { id:'fs-disk', cat:'forensics', name:'Imagem Disco', irreversible:true, args:[
    { name:'device', label:'Dispositivo', type:'text', required:true },
    { name:'out', label:'Saída', type:'text', required:true }
  ], build: v => ['forensics','--action','disk','--device', v.device, '--out', v.out] },
  { id:'rep-gen', cat:'reporting', name:'Gerar Relatório', args:[
    { name:'out', label:'Saída', type:'text', default:'pentest.md' }
  ], build: v => ['report','--out', v.out] },
];

// ═════════ STATE / VIEWS ═════════
const state = { activeModule:null, formValues:{}, openCats:new Set(['ascii']), view:'app' };
let pendingRun = null, lastData = null, activeTab = 'stdout';
const findingHistory = [];

const recents = JSON.parse(localStorage.getItem('axis_recents') || '[]');
function pushRecent(modId){
  const i = recents.indexOf(modId);
  if (i >= 0) recents.splice(i,1);
  recents.unshift(modId);
  recents.length = Math.min(recents.length, 5);
  localStorage.setItem('axis_recents', JSON.stringify(recents));
  renderRecents();
}
function renderRecents(){
  const host = document.getElementById('recents'); if (!host) return;
  const items = recents.map(id => MODULES.find(m => m.id === id)).filter(Boolean);
  host.innerHTML = items.length === 0 ? '' :
    `<div class="recents-label">recentes</div>` +
    items.map((m,i) => `
      <div class="recent-item" data-id="${m.id}">
        <span style="width:6px;height:6px;border-radius:50%;background:${catColor(m.cat)}"></span>
        ${m.name}<span class="kbd">${i+1}</span>
      </div>`).join('');
  host.querySelectorAll('.recent-item').forEach(el => {
    el.onclick = () => selectModule(MODULES.find(m => m.id === el.dataset.id));
  });
}
function renderSidebar(filter){
  const list = document.getElementById('cat-list');
  list.innerHTML = '';
  const f = (filter || '').toLowerCase();
  for (const cat of CATEGORIES){
    const catMods = MODULES.filter(m => m.cat === cat.id);
    const mods = f
      ? catMods.map(m => ({m, score: Math.max(fuzzyMatch(f, m.name), fuzzyMatch(f, m.id)*0.9)}))
              .filter(x => x.score > 0).sort((a,b)=>b.score-a.score).map(x=>x.m)
      : catMods;
    if (mods.length === 0) continue;
    const open = state.openCats.has(cat.id) || f !== '';
    const el = document.createElement('div');
    el.className = 'cat' + (open ? ' open' : '');
    const header = document.createElement('div');
    header.className = 'cat-header';
    header.innerHTML = `
      <span class="cat-dot" style="background:${cat.color};box-shadow:0 0 6px ${cat.color}"></span>
      <span class="cat-name">${cat.name}</span>
      <span class="cat-count">${mods.length}</span>
      <span class="cat-chevron">▶</span>`;
    header.onclick = () => {
      if (state.openCats.has(cat.id)) state.openCats.delete(cat.id);
      else state.openCats.add(cat.id);
      renderSidebar(document.getElementById('search').value);
    };
    el.appendChild(header);
    const body = document.createElement('div');
    body.className = 'cat-body';
    for (const mod of mods){
      const item = document.createElement('div');
      item.className = 'mod-item' + (state.activeModule?.id === mod.id ? ' active' : '');
      item.innerHTML = mod.name + (mod.irreversible ? '<span class="irrev">●</span>' : '');
      item.onclick = () => selectModule(mod);
      body.appendChild(item);
    }
    el.appendChild(body);
    list.appendChild(el);
  }
}
document.getElementById('search').addEventListener('input', e => renderSidebar(e.target.value));

function selectModule(mod){
  if (!mod) return;
  state.activeModule = mod;
  state.formValues = {};
  for (const a of mod.args) state.formValues[a.name] = a.default ?? '';
  pushRecent(mod.id);
  renderSidebar(document.getElementById('search').value);
  renderForm();
  clearOutput();
  if (state.view !== 'app') setView('app');
}
function renderForm(){
  const mod = state.activeModule;
  const title = document.getElementById('mod-title');
  const form = document.getElementById('form-grid');
  const actions = document.getElementById('mod-actions');
  form.innerHTML = ''; actions.innerHTML = '';
  if (!mod){
    title.textContent = 'selecione um módulo';
    form.innerHTML = '<div style="grid-column:1/-1;color:var(--text-dim);font-size:11px;padding:6px 0">← escolha um módulo na barra lateral</div>';
    return;
  }
  const color = catColor(mod.cat);
  title.innerHTML = `
    <span style="color:${color}">●</span>
    <span class="breadcrumb">
      <span class="bc-cat">${catName(mod.cat)}</span>
      <span class="bc-sep">›</span>
      <span class="bc-mod">${mod.name}</span>
    </span>
    ${mod.irreversible ? '<span class="irrev-tag">[SENSÍVEL]</span>' : ''}`;
  if (mod.args.length === 0){
    form.innerHTML = '<div style="grid-column:1/-1;color:var(--text-dim);font-size:11px;padding:4px 0">sem argumentos</div>';
  }
  for (const a of mod.args){
    const label = document.createElement('label');
    label.textContent = a.label + (a.required ? ' *' : '');
    let input;
    if (a.type === 'select'){
      input = document.createElement('select');
      for (const o of a.options){
        const opt = document.createElement('option');
        opt.value = o; opt.textContent = o;
        if (o === a.default) opt.selected = true;
        input.appendChild(opt);
      }
    } else {
      input = document.createElement('input');
      input.type = a.type;
      input.value = a.default ?? '';
    }
    input.name = a.name;
    const setter = () => { state.formValues[a.name] = input.value; };
    input.oninput = setter; input.onchange = setter;
    form.appendChild(label); form.appendChild(input);
    const hint = document.createElement('div');
    hint.className = 'field-hint';
    hint.textContent = a.required ? 'obrigatório' : '';
    form.appendChild(hint);
  }
  const runBtn = document.createElement('button');
  runBtn.className = 'action primary';
  runBtn.textContent = mod.irreversible ? 'EXECUTAR ▸ CONFIRMAR' : 'EXECUTAR ▸';
  runBtn.onclick = () => triggerRun(mod);
  actions.appendChild(runBtn);
  const clearBtn = document.createElement('button');
  clearBtn.className = 'action'; clearBtn.textContent = 'LIMPAR';
  clearBtn.onclick = clearOutput;
  actions.appendChild(clearBtn);
}

const history = JSON.parse(localStorage.getItem('axis_history') || '[]');
let histIdx = -1;
function pushHistory(entry){
  history.unshift(entry);
  history.length = Math.min(history.length, 50);
  localStorage.setItem('axis_history', JSON.stringify(history));
  histIdx = -1;
}

async function triggerRun(mod){
  const args = mod.build(state.formValues);
  const needsOpsecCheck = mod.irreversible || mod.sensitive || mod.cat === 'credential';
  if (!needsOpsecCheck){ executeRun(args, false); return; }
  const opsec = await fetchOpsecQuick();
  const showOpsecWarn = opsec.vulnerable && !opsecDismissedSession;
  if (!showOpsecWarn && !mod.irreversible){ executeRun(args, false); return; }
  const modal = document.getElementById('sensitive-modal');
  const title = document.getElementById('sens-title');
  const body = document.getElementById('sens-body');
  const irrevSection = document.getElementById('sens-irrev-section');
  const input = document.getElementById('sens-confirm-input');
  const goBtn = document.getElementById('sens-go');
  const auditBtn = document.getElementById('sens-audit');
  const configBtn = document.getElementById('sens-config');
  modal.classList.toggle('danger', !!mod.irreversible);
  const parts = [];
  if (mod.irreversible) parts.push('SENSÍVEL');
  if (showOpsecWarn) parts.push('POSTURA');
  title.textContent = '⚠ ' + parts.join(' · ');
  let bodyHtml = `<p>Comando:
    <code style="display:block;background:var(--surface-2);padding:6px 10px;border-radius:3px;color:var(--cyan);font-size:10px;margin-top:4px;word-break:break-all">axis ${args.join(' ')}</code></p>`;
  if (mod.irreversible){
    bodyHtml += `<p style="color:var(--red);font-size:11px;margin-top:8px">
      Esta operação escreve no sistema e passa pela política de escopo.</p>`;
  }
  if (showOpsecWarn){
    bodyHtml += `<p style="color:var(--yellow);font-size:11px;margin-top:12px"><b>Estado OPSEC:</b></p><ul>`;
    bodyHtml += opsec.scope_ok ? `<li class="ok-item">✓ scope.yaml presente</li>` : `<li class="bad-item">✗ scope.yaml ausente</li>`;
    bodyHtml += opsec.vpn_up ? `<li class="ok-item">✓ VPN detectada</li>` : `<li class="bad-item">✗ VPN não detectada</li>`;
    bodyHtml += opsec.mac_spoofed ? `<li class="ok-item">✓ MAC spoofado</li>` : `<li class="bad-item">✗ MAC não-spoofado</li>`;
    bodyHtml += `</ul>`;
    bodyHtml += `<label style="font-size:11px;display:flex;align-items:center;gap:6px;margin-top:10px;color:var(--text-dim);cursor:pointer">
      <input type="checkbox" id="sens-dismiss"> Não avisar novamente nesta sessão</label>`;
  }
  body.innerHTML = bodyHtml;
  irrevSection.hidden = !mod.irreversible;
  input.value = '';
  goBtn.disabled = mod.irreversible;
  goBtn.textContent = mod.irreversible ? 'EXECUTAR' : 'CONTINUAR';
  input.oninput = () => { goBtn.disabled = input.value.trim() !== 'EXECUTE'; };
  auditBtn.style.display = showOpsecWarn ? 'inline-flex' : 'none';
  configBtn.style.display = showOpsecWarn ? 'inline-flex' : 'none';
  const dismissCheck = () => {
    const cb = document.getElementById('sens-dismiss');
    if (cb && cb.checked) opsecDismissedSession = true;
  };
  goBtn.onclick = () => {
    if (mod.irreversible && input.value.trim() !== 'EXECUTE') return;
    dismissCheck(); modal.classList.remove('show');
    executeRun(args, mod.irreversible);
  };
  document.getElementById('sens-cancel').onclick = () => {
    modal.classList.remove('show'); pendingRun = null; toast('cancelado', 'warn');
  };
  auditBtn.onclick = () => { dismissCheck(); modal.classList.remove('show'); executeRun(['opsec','audit'], false); };
  configBtn.onclick = () => { modal.classList.remove('show'); setView('config'); };
  pendingRun = { mod, args };
  modal.classList.add('show');
  if (mod.irreversible) setTimeout(() => input.focus(), 50);
}
document.addEventListener('keydown', e => {
  if (e.key === 'Escape' && document.getElementById('sensitive-modal').classList.contains('show')){
    document.getElementById('sensitive-modal').classList.remove('show');
    pendingRun = null;
  }
});

async function executeRun(args, confirmed){
  const btn = state.activeModule && document.querySelector('.action.primary');
  const oldText = btn?.textContent;
  if (btn){ btn.classList.add('loading'); btn.textContent = 'EXECUTANDO'; }
  clearOutput();
  setOutput('executando: axis ' + args.join(' ') + '\n\n');
  document.getElementById('footer-cmd').textContent = 'executando…';
  pushHistory({ args, moduleId: state.activeModule?.id, ts: Date.now() });
  try {
    const res = await fetch('/api/run', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ args, confirm: confirmed })
    });
    const data = await res.json();
    lastData = data;
    renderOutput(data);
  } catch (e){
    setOutput('erro de fetch: ' + e.message);
    toast('erro de fetch — servidor off-line?', 'err');
  } finally {
    if (btn){ btn.classList.remove('loading'); btn.textContent = oldText; }
    document.getElementById('footer-cmd').textContent = 'pronto';
  }
}
function renderOutput(data){
  const stat = document.getElementById('out-stat');
  const stderrTab = document.querySelector('.out-tab[data-tab="stderr"]');
  if (stderrTab){
    if (data.stderr) stderrTab.removeAttribute('data-empty');
    else stderrTab.setAttribute('data-empty','1');
  }
  if (data.blocked){
    stat.textContent = 'bloqueado'; stat.style.color = 'var(--yellow)';
    renderTab(); toast(data.blocked, 'warn'); return;
  }
  stat.textContent = `exit ${data.code} · ${data.duration_ms}ms`;
  stat.style.color = data.code === 0 ? 'var(--green)' : 'var(--red)';
  renderTab();
}
function renderTab(){
  const pre = document.getElementById('out-pre');
  if (!lastData) return;
  pre.style.color = '';
  if (activeTab === 'stdout') pre.textContent = lastData.stdout || '(vazio)';
  else if (activeTab === 'stderr'){
    pre.textContent = lastData.stderr || '(vazio)';
    pre.style.color = lastData.stderr ? 'var(--red)' : 'var(--text-dim)';
  } else {
    pre.textContent = JSON.stringify(lastData, null, 2);
    pre.style.color = 'var(--text-dim)';
  }
}
document.querySelectorAll('.out-tab').forEach(tab => {
  tab.onclick = () => {
    activeTab = tab.dataset.tab;
    document.querySelectorAll('.out-tab').forEach(t => t.classList.toggle('active', t === tab));
    renderTab();
  };
});
document.getElementById('out-copy').onclick = async () => {
  if (!lastData) return;
  const txt = activeTab === 'stderr' ? lastData.stderr :
              activeTab === 'raw' ? JSON.stringify(lastData, null, 2) :
              (lastData.stdout || '');
  try { await navigator.clipboard.writeText(txt); toast('copiado'); }
  catch (_) { toast('falha ao copiar', 'err'); }
};
document.getElementById('out-dl').onclick = () => {
  if (!lastData) return;
  const blob = new Blob([lastData.stdout || ''], { type: 'text/plain' });
  const a = document.createElement('a');
  a.href = URL.createObjectURL(blob);
  a.download = `axis-${state.activeModule?.id || 'out'}-${Date.now()}.txt`;
  a.click(); URL.revokeObjectURL(a.href);
  toast('arquivo salvo');
};
document.getElementById('out-clear').onclick = clearOutput;
function setOutput(text){ document.getElementById('out-pre').textContent = text; }
function clearOutput(){
  lastData = null;
  document.getElementById('out-pre').textContent = '';
  document.getElementById('out-stat').textContent = '—';
  document.getElementById('out-stat').style.color = '';
  activeTab = 'stdout';
  document.querySelectorAll('.out-tab').forEach(t =>
    t.classList.toggle('active', t.dataset.tab === 'stdout'));
  const st = document.querySelector('.out-tab[data-tab="stderr"]');
  if (st) st.setAttribute('data-empty','1');
}

function toast(msg, kind = ''){
  const host = document.getElementById('toasts');
  const el = document.createElement('div');
  el.className = 'toast ' + kind;
  el.textContent = msg;
  host.appendChild(el);
  setTimeout(() => { el.classList.add('out'); setTimeout(() => el.remove(), 200); }, 2400);
}

function setView(v){
  state.view = v;
  document.getElementById('main-app').hidden = v !== 'app';
  document.getElementById('main-docs').hidden = v !== 'docs';
  document.getElementById('main-config').hidden = v !== 'config';
  document.getElementById('main-apis').hidden = v !== 'apis';
  document.getElementById('main-cli').hidden = v !== 'cli';
  document.getElementById('docs-btn').classList.toggle('active', v === 'docs');
  document.getElementById('config-btn').classList.toggle('active', v === 'config');
  document.getElementById('apis-btn').classList.toggle('active', v === 'apis');
  document.getElementById('cli-btn').classList.toggle('active', v === 'cli');
  if (v === 'docs') renderDocsCats();
  if (v === 'config') configReload();
  if (v === 'apis') apisReload();
  if (v === 'cli'){ initConsole(); setTimeout(() => document.getElementById('cli-in')?.focus(), 60); }
}
function toggleView(v){ setView(state.view === v ? 'app' : v); }
document.getElementById('docs-btn').onclick = () => toggleView('docs');
document.getElementById('config-btn').onclick = () => toggleView('config');
document.getElementById('apis-btn').onclick = () => toggleView('apis');
document.getElementById('cli-btn').onclick = () => toggleView('cli');
function renderDocsCats(){
  const host = document.getElementById('docs-cats');
  if (!host) return;
  host.innerHTML = '';
  for (const c of CATEGORIES){
    const count = MODULES.filter(m => m.cat === c.id).length;
    const div = document.createElement('div');
    div.className = 'docs-cat';
    div.innerHTML = `<span class="swatch" style="background:${c.color};box-shadow:0 0 5px ${c.color}"></span>${c.name}<span class="count">${count}</span>`;
    host.appendChild(div);
  }
}

// ═════════ PALETTE ═════════
let paletteIdx = 0, paletteHits = [];
function openPalette(){
  document.getElementById('palette').classList.add('show');
  const inp = document.getElementById('palette-input');
  inp.value = ''; inp.focus();
  runPaletteFilter('');
}
function closePalette(){ document.getElementById('palette').classList.remove('show'); }
function runPaletteFilter(q){
  const scored = MODULES.map(m => ({
    m, score: q ? Math.max(fuzzyMatch(q, m.name), fuzzyMatch(q, m.id)*0.9, fuzzyMatch(q, m.cat)*0.5) : 1,
  }));
  paletteHits = scored.filter(x => x.score > 0).sort((a,b) => b.score - a.score).slice(0,12).map(x => x.m);
  paletteIdx = 0;
  const list = document.getElementById('palette-list');
  list.innerHTML = paletteHits.map((m,i) => `
    <div class="palette-item ${i===0?'sel':''}" data-id="${m.id}">
      <span class="cat-dot" style="width:6px;height:6px;border-radius:50%;background:${catColor(m.cat)}"></span>
      ${m.name}<span class="cat-name">${catName(m.cat)}</span>
    </div>`).join('');
  list.querySelectorAll('.palette-item').forEach(el => {
    el.onclick = () => { selectModule(MODULES.find(m => m.id === el.dataset.id)); closePalette(); };
  });
}
function updatePaletteSel(){
  document.querySelectorAll('.palette-item').forEach((el,i) => el.classList.toggle('sel', i === paletteIdx));
}
document.getElementById('palette-input').oninput = e => runPaletteFilter(e.target.value);
document.getElementById('palette').onclick = e => { if (e.target.id === 'palette') closePalette(); };
document.addEventListener('keydown', e => {
  if ((e.metaKey || e.ctrlKey) && e.key === 'k'){
    e.preventDefault();
    document.getElementById('palette').classList.contains('show') ? closePalette() : openPalette();
    return;
  }
  if (document.getElementById('palette').classList.contains('show')){
    if (e.key === 'Escape'){ closePalette(); return; }
    if (e.key === 'ArrowDown'){ e.preventDefault(); paletteIdx = (paletteIdx+1)%Math.max(paletteHits.length,1); updatePaletteSel(); }
    if (e.key === 'ArrowUp'){ e.preventDefault(); paletteIdx = (paletteIdx-1+paletteHits.length)%Math.max(paletteHits.length,1); updatePaletteSel(); }
    if (e.key === 'Enter' && paletteHits[paletteIdx]){ selectModule(paletteHits[paletteIdx]); closePalette(); }
  }
});

// ═════════ HEADER STATE ═════════
async function refreshState(){
  try {
    const s = await (await fetch('/api/state')).json();
    const scope = document.getElementById('pill-scope');
    if (s.scope){
      scope.querySelector('.value').textContent = s.scope.engagement || `${s.scope.targets.length} alvos`;
      scope.className = 'pill ok';
    } else {
      scope.querySelector('.value').textContent = 'ausente';
      scope.className = 'pill bad';
    }
    const f = document.getElementById('pill-findings');
    findingHistory.push(s.findings);
    if (findingHistory.length > 20) findingHistory.shift();
    f.querySelector('.value').textContent = s.findings;
    f.className = 'pill ' + (s.findings > 0 ? 'warn' : 'ok');
    const h = document.getElementById('pill-host');
    h.querySelector('.value').textContent = s.hostname;
    h.className = 'pill ok';
    // SENTINEL health
    try {
      const hd = await (await fetch('/api/sentinel/health')).json();
      const hEl = document.getElementById('pill-sentinel');
      if (hd.ok){
        hEl.querySelector('.value').textContent = hd.incidents > 0 ? `${hd.incidents} inc` : `${hd.plugins}p`;
        hEl.className = 'pill ' + (hd.incidents > 0 ? 'warn' : 'ok');
      } else {
        hEl.querySelector('.value').textContent = 'off';
        hEl.className = 'pill bad';
      }
    } catch(_){
      const hEl = document.getElementById('pill-sentinel');
      if (hEl){ hEl.querySelector('.value').textContent = 'off'; hEl.className = 'pill bad'; }
    }
  } catch (_) {}
}
function tickUptime(){
  const uptime = Math.floor((Date.now() - appStart) / 1000);
  const el = document.getElementById('footer-cmd');
  if (el.textContent === 'pronto' || el.textContent.startsWith('pronto')){
    el.textContent = `pronto · uptime ${uptime}s`;
  }
}
function lev(a,b){
  const m = a.length, n = b.length;
  if (m === 0) return n; if (n === 0) return m;
  let prev = new Array(n+1), cur = new Array(n+1);
  for (let j = 0; j <= n; j++) prev[j] = j;
  for (let i = 1; i <= m; i++){
    cur[0] = i;
    for (let j = 1; j <= n; j++){
      cur[j] = a[i-1] === b[j-1] ? prev[j-1] : 1 + Math.min(prev[j], cur[j-1], prev[j-1]);
    }
    [prev, cur] = [cur, prev];
  }
  return prev[n];
}
function fuzzyMatch(query, target){
  if (!query) return 1;
  if (!target) return 0;
  const q = String(query).toLowerCase(), t = String(target).toLowerCase();
  if (t === q) return 1000;
  if (t.startsWith(q)) return 500 - (t.length - q.length);
  const idx = t.indexOf(q);
  if (idx >= 0) return 300 - idx - (t.length - q.length);
  let qi = 0, gaps = 0, lastMatch = -1;
  for (let i = 0; i < t.length && qi < q.length; i++){
    if (t[i] === q[qi]){ if (lastMatch >= 0) gaps += i - lastMatch - 1; lastMatch = i; qi++; }
  }
  if (qi === q.length) return Math.max(1, 100 - gaps*3 - (t.length - q.length));
  const d = lev(q, t);
  if (d <= 3) return Math.max(1, 40 - d*10);
  return 0;
}

let opsecDismissedSession = false;
async function fetchOpsecQuick(){
  try { return await (await fetch('/api/opsec-quick')).json(); }
  catch (_) { return { vulnerable:true, scope_ok:false, vpn_up:false, mac_spoofed:false }; }
}

// ═════════ SYSTEM VIEW ═════════
async function configReload(){
  try {
    const c = await (await fetch('/api/config')).json();
    document.getElementById('cfg-port').value = c.gui_port || '';
    document.getElementById('cfg-bin').value = c.axis_bin || '';
    document.getElementById('cfg-scope').value = c.scope_yaml || '';
    configRefreshProcs();
    configRefreshOpsec();
  } catch (e){ toast('erro ao carregar: ' + e.message, 'err'); }
}
async function configSaveScope(){
  const scope = document.getElementById('cfg-scope').value;
  try {
    const r = await fetch('/api/config', {
      method: 'POST', headers: { 'Content-Type':'application/json' },
      body: JSON.stringify({ scope_yaml: scope }),
    });
    const d = await r.json();
    if (d.ok){
      const el = document.getElementById('cfg-save-status');
      el.textContent = '✓ salvo ' + new Date().toLocaleTimeString();
      el.style.color = 'var(--green)';
      toast('escopo salvo');
    }
  } catch (e){ toast('erro: ' + e.message, 'err'); }
}
async function configServerTest(){
  try {
    const r = await fetch('/api/run', {
      method:'POST', headers:{'Content-Type':'application/json'},
      body: JSON.stringify({ args:['--version'], confirm:false }),
    });
    const d = await r.json();
    if (d.code === 0) toast('binário OK');
    else toast('falhou: ' + (d.stderr || 'exit ' + d.code), 'err');
  } catch (e){ toast('erro: ' + e.message, 'err'); }
}
async function configSpawnServer(name, args){
  try {
    const r = await fetch('/api/spawn', {
      method:'POST', headers:{'Content-Type':'application/json'},
      body: JSON.stringify({ name, args }),
    });
    const d = await r.json();
    if (d.spawned) toast(`${name} iniciado (pid ${d.pid})`);
    else toast(`falha: ${d.error}`, 'err');
    configRefreshProcs();
  } catch (e){ toast('erro: ' + e.message, 'err'); }
}
async function configKillAll(){
  try {
    const d = await (await fetch('/api/processes')).json();
    for (const p of (d.processes || [])){
      await fetch('/api/kill', {
        method:'POST', headers:{'Content-Type':'application/json'},
        body: JSON.stringify({ name: p.name }),
      });
    }
    toast('processos parados');
    configRefreshProcs();
  } catch (e){ toast('erro: ' + e.message, 'err'); }
}
async function configRefreshProcs(){
  try {
    const d = await (await fetch('/api/processes')).json();
    const host = document.getElementById('cfg-procs');
    if (!host) return;
    if (!d.processes || d.processes.length === 0){
      host.innerHTML = '<span style="color:var(--text-dim)">nenhum</span>';
    } else {
      host.innerHTML = d.processes.map(p =>
        `<span class="cfg-proc">${p.name} <span style="color:var(--text-dim)">pid ${p.pid}</span></span>`).join('');
    }
  } catch (_) {}
}
async function configRefreshOpsec(){
  const d = await fetchOpsecQuick();
  const set = (id, ok, yes, no) => {
    const el = document.getElementById(id); if (!el) return;
    el.textContent = ok ? yes : no;
    el.className = 'cfg-pill ' + (ok ? 'ok' : 'bad');
  };
  set('cfg-status-scope', d.scope_ok, '✓ scope', '✗ scope');
  set('cfg-status-vpn', d.vpn_up, '✓ vpn', '✗ vpn');
  set('cfg-status-mac', d.mac_spoofed, '✓ mac', '✗ mac');
}
function configRun(args, opts = {}){
  const mod = {
    id: 'config-' + (args[0] || 'custom'),
    cat: opts.cat || 'config',
    name: opts.name || ('config ' + args.slice(0,2).join(' ')),
    irreversible: !!opts.irreversible,
    sensitive: !!opts.sensitive,
    args: [], build: () => args,
  };
  triggerRun(mod);
}
setInterval(() => { if (state.view === 'config') configRefreshProcs(); }, 3000);

// ═════════ SENTINEL control (from sistema tab) ═════════
async function sentinelHealth(){
  const el = document.getElementById('sentinel-status');
  if (el) el.textContent = 'consultando…';
  try {
    const r = await fetch('/api/sentinel/health');
    const d = await r.json();
    if (el) el.textContent = JSON.stringify(d, null, 2);
  } catch (e){
    if (el) el.textContent = 'erro: ' + e.message;
  }
}
async function sentinelStart(){
  const el = document.getElementById('sentinel-status');
  if (el) el.textContent = 'iniciando…';
  try {
    const r = await fetch('/api/run', {
      method:'POST', headers:{'Content-Type':'application/json'},
      body: JSON.stringify({ args:['defense','--action','sentinel-start'], confirm:false }),
    });
    const d = await r.json();
    if (el) el.textContent = d.stdout || d.stderr || `exit ${d.code}`;
    toast('sentinel start enviado');
    refreshState();
  } catch (e){ if (el) el.textContent = 'erro: ' + e.message; }
}
async function sentinelStop(){
  const el = document.getElementById('sentinel-status');
  if (el) el.textContent = 'parando…';
  try {
    const r = await fetch('/api/run', {
      method:'POST', headers:{'Content-Type':'application/json'},
      body: JSON.stringify({ args:['defense','--action','sentinel-stop'], confirm:false }),
    });
    const d = await r.json();
    if (el) el.textContent = d.stdout || d.stderr || `exit ${d.code}`;
    toast('sentinel stop enviado', 'warn');
    refreshState();
  } catch (e){ if (el) el.textContent = 'erro: ' + e.message; }
}

// ═════════ APIS VIEW ═════════
async function apisReload(){
  try {
    const c = await (await fetch('/api/config')).json();
    const env = c.env || {};
    document.querySelectorAll('#main-apis .apis-card').forEach(card => {
      const key = card.dataset.key;
      const input = card.querySelector(`input[data-for="${key}"]`);
      const status = card.querySelector(`.bind-status[data-for="${key}"]`);
      if (!input || !status) return;
      const v = env[key];
      input.value = '';
      if (v !== undefined && v !== ''){
        status.textContent = '✓ ' + v;
        status.className = 'bind-status ok';
      } else {
        status.textContent = '✗ unbound';
        status.className = 'bind-status bad';
      }
    });
  } catch (e){ toast('erro ao carregar: ' + e.message, 'err'); }
}
async function apisBind(key, value){
  const patch = {}; patch[key] = value || '';
  try {
    const r = await fetch('/api/config', {
      method:'POST', headers:{'Content-Type':'application/json'},
      body: JSON.stringify({ env: patch }),
    });
    const d = await r.json();
    if (d.ok){
      const el = document.getElementById('apis-status');
      el.textContent = '✓ ' + key + ' ' + (value ? 'gravado' : 'removido') + ' · ' + new Date().toLocaleTimeString();
      el.style.color = 'var(--green)';
      toast('✓ ' + key, 'aura');
      apisReload();
    }
  } catch (e){ toast('erro ao salvar: ' + e.message, 'err'); }
}
document.querySelectorAll('#main-apis .apis-actions .action').forEach(btn => {
  btn.onclick = () => {
    const key = btn.dataset.key;
    const card = btn.closest('.apis-card');
    const input = card.querySelector(`input[data-for="${key}"]`);
    if (btn.dataset.act === 'bind'){
      const val = input.value.trim();
      if (!val){ toast('digite um valor', 'warn'); return; }
      apisBind(key, val);
    }
  };
});

// ═════════ CLI VIEW — AURA ═════════
const CLI_COMMANDS = [
  'help','clear','set','get','del','list','keys','scope','start','stop','ps','run','audit','opsec','exfil',
  'aura','aura help','aura init','aura bind','aura unbanish','aura whoami','aura vision','aura ping',
  'aura ritual','aura themes','aura theme','aura export','aura wipe','aura crown','aura lock',
];
const CONFIG_KEYS = [
  'HIBP_KEY','SHODAN_KEY','VT_KEY','GITHUB_TOKEN','AXIS_STEALER_TG_TOKEN','AXIS_STEALER_TG_CHAT',
  'AXIS_STEALER_DISCORD_WEBHOOK','AXIS_CF_WORKER','AXIS_PROXY_DEFAULT',
  'AXIS_BIN','AXIS_GUI_PORT','AXIS_C2_PORT','AXIS_C2_URL',
];
const AURA_SERVICES = {
  hibp:'HIBP_KEY', shodan:'SHODAN_KEY', vt:'VT_KEY', virustotal:'VT_KEY',
  github:'GITHUB_TOKEN', 'telegram-token':'AXIS_STEALER_TG_TOKEN',
  'telegram-chat':'AXIS_STEALER_TG_CHAT', discord:'AXIS_STEALER_DISCORD_WEBHOOK',
  cf:'AXIS_CF_WORKER', cloudflare:'AXIS_CF_WORKER', proxy:'AXIS_PROXY_DEFAULT',
};
const cliHistory = [];
let cliHistIdx = -1, cliSugIdx = 0, cliSugs = [];
function cliLog(text, cls = ''){
  const host = document.getElementById('cli-out'); if (!host) return;
  text.split('\n').forEach(line => {
    const el = document.createElement('div');
    el.className = 'cli-line ' + cls;
    el.textContent = line;
    host.appendChild(el);
  });
  host.scrollTop = host.scrollHeight;
}
function cliSuggestReset(){
  const s = document.getElementById('cli-suggest'); if (!s) return;
  s.classList.remove('show'); s.innerHTML = '';
  cliSugs = []; cliSugIdx = 0;
}
function computeCliSugs(line, cursor){
  const before = line.slice(0, cursor);
  const tokens = before.split(/\s+/);
  const last = tokens[tokens.length - 1] || '';
  const first = (tokens[0] || '').toLowerCase();
  if (tokens.length === 1){
    return CLI_COMMANDS.map(c => ({ full:c, prefix:'', score:fuzzyMatch(last,c) }))
      .filter(x => x.score > 0).sort((a,b)=>b.score-a.score).slice(0,8);
  }
  if (['set','get','del'].includes(first) && tokens.length === 2 && !before.endsWith(' ')){
    return CONFIG_KEYS.map(k => ({ full:k, prefix:first+' ', score:fuzzyMatch(tokens[1],k) }))
      .filter(x => x.score > 0).sort((a,b)=>b.score-a.score).slice(0,8);
  }
  if (first === 'aura'){
    if (tokens.length === 2 && !before.endsWith(' ')){
      return ['help','init','bind','unbanish','whoami','vision','ping','ritual','themes','theme','export','wipe','crown','lock']
        .map(c => ({ full:c, prefix:'aura ', score:fuzzyMatch(tokens[1],c) }))
        .filter(x => x.score > 0).sort((a,b)=>b.score-a.score);
    }
    if (tokens.length === 3 && tokens[1] === 'bind' && !before.endsWith(' ')){
      return Object.keys(AURA_SERVICES).map(s => ({ full:s, prefix:'aura bind ', score:fuzzyMatch(tokens[2],s) }))
        .filter(x => x.score > 0).sort((a,b)=>b.score-a.score);
    }
    if (tokens.length === 3 && tokens[1] === 'theme' && !before.endsWith(' ')){
      return Object.keys(THEMES).map(t => ({ full:t, prefix:'aura theme ', score:fuzzyMatch(tokens[2],t) }))
        .filter(x => x.score > 0).sort((a,b)=>b.score-a.score);
    }
  }
  return [];
}
function renderCliSugs(){
  const host = document.getElementById('cli-suggest'); if (!host) return;
  if (cliSugs.length === 0){ cliSuggestReset(); return; }
  host.classList.add('show');
  host.innerHTML = cliSugs.map((s,i) => `
    <div class="cli-sug ${i===cliSugIdx?'sel':''}" data-i="${i}">
      <span class="sug-prefix">${s.prefix || ''}</span>
      <span>${s.full}</span>
      <span class="sug-arrow">${i===cliSugIdx?'↵':''}</span>
    </div>`).join('');
  host.querySelectorAll('.cli-sug').forEach(el => {
    el.onclick = () => acceptCliSug(parseInt(el.dataset.i));
  });
}
function acceptCliSug(idx){
  const sug = cliSugs[idx]; if (!sug) return;
  const input = document.getElementById('cli-in');
  const line = input.value, pos = input.selectionStart;
  const before = line.slice(0,pos), after = line.slice(pos);
  const toks = before.split(/\s+/);
  const last = toks[toks.length-1] || '';
  const replaced = before.slice(0, before.length - last.length) + sug.full;
  input.value = replaced + (after ? ' ' + after.trimStart() : ' ');
  input.focus();
  input.setSelectionRange(input.value.length, input.value.length);
  cliSuggestReset();
}
function auraBanner(text){
  cliLog('');
  cliLog('  ╔══════════════════════════════════════════╗', 'aura');
  cliLog('  ║  ' + text.padEnd(40) + '║', 'aura');
  cliLog('  ╚══════════════════════════════════════════╝', 'aura');
  cliLog('');
}
function auraCrown(){
  const op = (localStorage.getItem('axis_operator') || 'ANON').toUpperCase();
  const line = '✦ AXIS · ' + op + ' ✦';
  cliLog('');
  cliLog('  ' + '━'.repeat(line.length), 'aura');
  cliLog('  ' + line, 'aura');
  cliLog('  ' + '━'.repeat(line.length), 'aura');
  cliLog('');
}
async function auraInit(){
  auraBanner('AXIS · ONBOARDING');
  const c = await (await fetch('/api/config')).json();
  const env = c.env || {};
  const missing = [];
  for (const [svc, key] of Object.entries(AURA_SERVICES)){
    if (!env[key]) missing.push({ svc, key });
  }
  if (missing.length === 0) cliLog('  ✓ tudo configurado.', 'ok');
  else {
    cliLog('  slots em aberto:', 'dim');
    for (const m of missing) cliLog(`    aura bind ${m.svc.padEnd(15)} <valor>`, 'dim');
  }
  cliLog('');
}
async function auraBind(svc, value){
  const key = AURA_SERVICES[svc];
  if (!key){ cliLog(`  ✗ serviço desconhecido: ${svc}`, 'err'); return; }
  if (!value){ cliLog(`  ✗ uso: aura bind ${svc} <valor>`, 'err'); return; }
  await configSaveEnv({ [key]: value });
  cliLog(`  ✓ ${key} gravado`, 'aura');
  toast('✦ ' + key, 'aura');
}
async function auraUnbanish(svc){
  const key = AURA_SERVICES[svc];
  if (!key){ cliLog(`  ✗ desconhecido: ${svc}`, 'err'); return; }
  await configSaveEnv({ [key]: '' });
  cliLog(`  ✓ ${key} removido`, 'aura');
}
async function auraWhoami(){
  const op = localStorage.getItem('axis_operator') || 'anon';
  const theme = localStorage.getItem('axis_theme') || 'axis';
  const s = await (await fetch('/api/state')).json();
  auraCrown();
  cliLog('  operator:  ' + op, 'aura');
  cliLog('  host:      ' + (s.hostname || '?'), 'aura');
  cliLog('  theme:     ' + theme, 'aura');
  cliLog('  escopo:    ' + (s.scope ? (s.scope.engagement || `${s.scope.targets.length} alvos`) : '✗ ausente'), 'aura');
  cliLog('  findings:  ' + (s.findings ?? 0), 'aura');
  cliLog('');
}
async function auraVision(){
  const s = await (await fetch('/api/state')).json();
  const o = await fetchOpsecQuick();
  const theme = localStorage.getItem('axis_theme') || 'axis';
  const mark = b => b ? '✓' : '✗';
  cliLog('');
  cliLog('  ┌─ HOST ─────────────────┐  ┌─ OPSEC ──────────────┐', 'aura');
  cliLog(`  │ name:  ${(s.hostname||'?').padEnd(15)}│  │ scope: ${mark(o.scope_ok)}            │`, 'aura');
  cliLog(`  │ theme: ${theme.padEnd(15)}│  │ vpn:   ${mark(o.vpn_up)}            │`, 'aura');
  cliLog(`  │ finds: ${String(s.findings??0).padEnd(15)}│  │ mac:   ${mark(o.mac_spoofed)}            │`, 'aura');
  cliLog('  └────────────────────────┘  └──────────────────────┘', 'aura');
  cliLog('');
}
async function auraPing(){
  cliLog('  ▸ axis opsec audit', 'aura');
  const d = await (await fetch('/api/run', {
    method:'POST', headers:{'Content-Type':'application/json'},
    body: JSON.stringify({ args:['opsec','audit'], confirm:false }),
  })).json();
  if (d.blocked) cliLog('  ⚠ ' + d.blocked, 'warn');
  else {
    if (d.stdout) cliLog(d.stdout, 'ok');
    if (d.stderr) cliLog(d.stderr, 'err');
    cliLog(`  exit ${d.code} · ${d.duration_ms}ms`, 'dim');
  }
}
async function auraRitual(){
  auraBanner('RITUAL · OPSEC');
  cliLog('  [1/3] auditoria...', 'dim');
  await auraPing();
  const o = await fetchOpsecQuick();
  cliLog('');
  if (!o.vpn_up){
    cliLog('  [2/3] VPN down', 'warn');
    if (confirm('Ativar killswitch agora?')){
      configRun(['opsec','killswitch','enable','--iface','WireGuard'], { name:'Ativar killswitch', irreversible:true });
    }
  } else cliLog('  [2/3] ✓ VPN up', 'ok');
  if (!o.mac_spoofed){
    cliLog('  [3/3] MAC não-spoofado', 'warn');
    if (confirm('Rotacionar MAC?')){
      configRun(['opsec','mac','set','Wi-Fi'], { name:'Rotacionar MAC', irreversible:true });
    }
  } else cliLog('  [3/3] ✓ MAC spoofado', 'ok');
  cliLog('');
  cliLog('  ✦ ritual concluído', 'aura');
}
function auraThemes(){
  const cur = localStorage.getItem('axis_theme') || 'axis';
  cliLog('');
  cliLog('  paletas:', 'aura');
  for (const [k, v] of Object.entries(THEMES)){
    cliLog(`    ${k.padEnd(8)}  cyan ${v.cyan}  magenta ${v.magenta}${k===cur?' ← ativa':''}`, 'dim');
  }
  cliLog('');
}
function auraTheme(name){
  if (!name){ auraThemes(); return; }
  if (!THEMES[name]){ cliLog(`  ✗ tema: ${name}`, 'err'); return; }
  applyTheme(name);
  cliLog(`  ✓ tema → ${name}`, 'aura');
  toast('✦ theme: ' + name, 'aura');
}
async function auraExport(){
  const c = await (await fetch('/api/config')).json();
  cliLog('  ── export ──', 'aura');
  cliLog(JSON.stringify({
    env: c.env || {},
    scope_yaml: c.scope_yaml || '',
    theme: localStorage.getItem('axis_theme') || 'axis',
    operator: localStorage.getItem('axis_operator') || 'anon',
    ts: new Date().toISOString(),
  }, null, 2), 'dim');
  cliLog('  ── fim ──', 'aura');
}
async function auraWipe(){
  cliLog('  ⚠ apaga TODAS as chaves', 'err');
  if (!confirm('Confirmar aura wipe?')){ cliLog('  cancelado.', 'dim'); return; }
  const patch = {};
  for (const k of Object.values(AURA_SERVICES)) patch[k] = '';
  await configSaveEnv(patch);
  cliLog('  ✦ chaves apagadas.', 'aura');
}
async function configSaveEnv(patch){
  const r = await fetch('/api/config', {
    method:'POST', headers:{'Content-Type':'application/json'},
    body: JSON.stringify({ env: patch }),
  });
  return r.json();
}
async function execCli(raw){
  const line = raw.trim(); if (!line) return;
  cliLog(line, 'echo');
  cliHistory.unshift(line);
  cliHistory.length = Math.min(cliHistory.length, 50);
  cliHistIdx = -1;
  const parts = line.match(/(?:"[^"]*"|'[^']*'|\S+)/g) || [];
  const cmd = (parts.shift() || '').toLowerCase();
  const args = parts.map(p => p.replace(/^["']|["']$/g, ''));
  try {
    if (cmd === 'aura'){
      const sub = (args[0] || '').toLowerCase();
      switch (sub){
        case '': case 'help': {
          auraBanner('AURA · CONFIG');
          cliLog('  bind       aura bind <svc> <valor>', 'dim');
          cliLog('  unbanish   aura unbanish <svc>', 'dim');
          cliLog('  init       onboarding', 'dim');
          cliLog('  whoami     identidade', 'dim');
          cliLog('  vision     dashboard', 'dim');
          cliLog('  ping       opsec audit', 'dim');
          cliLog('  ritual     OPSEC sequencial', 'dim');
          cliLog('  themes     lista paletas', 'dim');
          cliLog('  theme      aura theme <nome>', 'dim');
          cliLog('  export     dump JSON', 'dim');
          cliLog('  wipe       apaga chaves', 'dim');
          cliLog('  crown      banner', 'dim');
          cliLog('  lock       logout', 'dim');
          return;
        }
        case 'init': await auraInit(); return;
        case 'bind': await auraBind(args[1], args.slice(2).join(' ')); return;
        case 'unbanish': await auraUnbanish(args[1]); return;
        case 'whoami': await auraWhoami(); return;
        case 'vision': await auraVision(); return;
        case 'ping': await auraPing(); return;
        case 'ritual': await auraRitual(); return;
        case 'themes': auraThemes(); return;
        case 'theme': auraTheme(args[1]); return;
        case 'export': await auraExport(); return;
        case 'wipe': await auraWipe(); return;
        case 'crown': auraCrown(); return;
        case 'lock': cliLog('  ✦ sessão encerrada.', 'aura');
          setTimeout(() => { localStorage.removeItem('axis_auth'); location.reload(); }, 700); return;
        default: cliLog(`  ✗ aura ${sub} desconhecido`, 'err'); return;
      }
    }
    switch (cmd){
      case 'help':
        auraBanner('BASE · COMMANDS');
        cliLog([
          '  set <KEY> <VALUE>     define em %USERPROFILE%\\.axis\\env',
          '  get <KEY>             mostra valor',
          '  del <KEY>             remove',
          '  list | keys           lista variáveis',
          '  scope                 mostra scope.yaml',
          '  ps                    lista background',
          '  run <args...>         executa axis',
          '  audit | opsec         roda opsec audit',
          '  clear                 limpa console',
          '',
          '  aura help             comandos AURA',
        ].join('\n'), 'dim'); return;
      case 'clear': document.getElementById('cli-out').innerHTML = ''; return;
      case 'set': {
        if (args.length < 2) throw new Error('uso: set <KEY> <VALUE>');
        await configSaveEnv({ [args[0]]: args.slice(1).join(' ') });
        cliLog(`  ✓ ${args[0]} definido`, 'ok'); return;
      }
      case 'get': {
        if (args.length < 1) throw new Error('uso: get <KEY>');
        const c = await (await fetch('/api/config')).json();
        const v = (c.env || {})[args[0]];
        cliLog(v === undefined ? `  ${args[0]} não definido` : `  ${args[0]} = ${v}`, v === undefined ? 'warn' : 'ok');
        return;
      }
      case 'del': {
        if (args.length < 1) throw new Error('uso: del <KEY>');
        await configSaveEnv({ [args[0]]: '' });
        cliLog(`  ✓ ${args[0]} removido`, 'ok'); return;
      }
      case 'list': case 'keys': {
        const c = await (await fetch('/api/config')).json();
        const keys = Object.keys(c.env || {}).sort();
        if (keys.length === 0) cliLog('  (vazio)', 'dim');
        else keys.forEach(k => cliLog(`    ${k.padEnd(28)} ${c.env[k]}`, 'dim'));
        return;
      }
      case 'scope': {
        const c = await (await fetch('/api/config')).json();
        cliLog(c.scope_yaml || '  (vazio)', 'dim'); return;
      }
      case 'ps': {
        const d = await (await fetch('/api/processes')).json();
        if (!d.processes || d.processes.length === 0) cliLog('  (nenhum)', 'dim');
        else d.processes.forEach(p => cliLog(`    ${p.name.padEnd(12)} pid ${p.pid}`, 'dim'));
        return;
      }
      case 'run': {
        if (args.length < 1) throw new Error('uso: run <args...>');
        const d = await (await fetch('/api/run', {
          method:'POST', headers:{'Content-Type':'application/json'},
          body: JSON.stringify({ args, confirm:false }),
        })).json();
        if (d.blocked) cliLog(`  ⚠ ${d.blocked}`, 'warn');
        else {
          if (d.stdout) cliLog(d.stdout, 'ok');
          if (d.stderr) cliLog(d.stderr, 'err');
          cliLog(`  exit ${d.code} · ${d.duration_ms}ms`, 'dim');
        }
        return;
      }
      case 'audit': case 'opsec': { await auraPing(); return; }
      default: {
        const d = await (await fetch('/api/run', {
          method:'POST', headers:{'Content-Type':'application/json'},
          body: JSON.stringify({ args:[cmd, ...args], confirm:false }),
        })).json();
        if (d.blocked) cliLog(`  ⚠ ${d.blocked}`, 'warn');
        else {
          if (d.stdout) cliLog(d.stdout, 'ok');
          if (d.stderr) cliLog(d.stderr, 'err');
          cliLog(`  exit ${d.code} · ${d.duration_ms}ms`, 'dim');
        }
      }
    }
  } catch (e){ cliLog('  ✗ ' + e.message, 'err'); }
}
function initConsole(){
  const input = document.getElementById('cli-in');
  if (!input || input.dataset.bound) return;
  input.dataset.bound = '1';
  input.addEventListener('input', () => {
    cliSugs = computeCliSugs(input.value, input.selectionStart);
    cliSugIdx = 0;
    renderCliSugs();
  });
  input.addEventListener('keydown', e => {
    if (cliSugs.length > 0){
      if (e.key === 'ArrowDown'){ e.preventDefault(); cliSugIdx = (cliSugIdx+1)%cliSugs.length; renderCliSugs(); return; }
      if (e.key === 'ArrowUp'){ e.preventDefault(); cliSugIdx = (cliSugIdx-1+cliSugs.length)%cliSugs.length; renderCliSugs(); return; }
      if (e.key === 'Tab'){ e.preventDefault(); acceptCliSug(cliSugIdx); return; }
      if (e.key === 'Escape'){ e.preventDefault(); cliSuggestReset(); return; }
    }
    if (e.key === 'ArrowUp' && cliSugs.length === 0){
      if (cliHistory.length === 0) return;
      e.preventDefault();
      cliHistIdx = Math.min(cliHistIdx + 1, cliHistory.length - 1);
      input.value = cliHistory[cliHistIdx] || '';
      setTimeout(() => input.setSelectionRange(input.value.length, input.value.length), 0);
      return;
    }
    if (e.key === 'ArrowDown' && cliSugs.length === 0){
      if (cliHistIdx <= 0) return;
      e.preventDefault();
      cliHistIdx--;
      input.value = cliHistory[cliHistIdx] || '';
      return;
    }
    if (e.key === 'Enter'){
      e.preventDefault();
      const line = input.value;
      input.value = '';
      cliSuggestReset();
      execCli(line);
    }
  });
  if (!document.getElementById('cli-out').dataset.booted){
    document.getElementById('cli-out').dataset.booted = '1';
    auraBanner('CONSOLE · AURA');
    cliLog('  help   comandos base', 'dim');
    cliLog('  aura   comandos AURA', 'dim');
    cliLog('');
  }
}
document.querySelectorAll('.cli-chip').forEach(chip => {
  chip.onclick = () => {
    const cmd = chip.dataset.cmd; if (!cmd) return;
    initConsole();
    execCli(cmd);
  };
});

if (localStorage.getItem('axis_auth') === '1') showApp(); else bootLogin();
</script>
</body>
</html>"##;

// ═══════════════════════════════════════════════════════════════════════
// SERVER STATE
// ═══════════════════════════════════════════════════════════════════════
#[derive(Clone)]
struct AppState {
    axis_bin: String,
    scope: Arc<Mutex<Option<ScopeState>>>,
    config_dir: PathBuf,
    processes: Arc<Mutex<HashMap<String, u32>>>,
    opsec_cache: Arc<Mutex<Option<(Instant, serde_json::Value)>>>,
    http: reqwest::Client,
}

#[derive(Serialize, Clone)]
struct ScopeState {
    engagement: String,
    targets: Vec<String>,
    window_start: String,
    window_end: String,
}

#[derive(Deserialize)]
struct RunReq { args: Vec<String>, confirm: Option<bool> }

#[derive(Serialize)]
struct RunResp {
    stdout: String, stderr: String, code: i32, duration_ms: u128,
    blocked: Option<String>, needs_confirm: Option<bool>,
}

#[derive(Deserialize)]
struct ConfigReq { env: Option<HashMap<String, String>>, scope_yaml: Option<String> }

#[derive(Deserialize)]
struct SpawnReq { name: String, args: Vec<String> }

#[derive(Deserialize)]
struct KillReq { name: String }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port: u16 = std::env::var("AXIS_GUI_PORT").ok()
        .and_then(|s| s.parse().ok()).unwrap_or(7878);
    let axis_bin = std::env::var("AXIS_BIN").unwrap_or_else(|_| "axis".into());

    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()?;

    let state = AppState {
        axis_bin,
        scope: Arc::new(Mutex::new(load_scope())),
        config_dir: config_dir(),
        processes: Arc::new(Mutex::new(HashMap::new())),
        opsec_cache: Arc::new(Mutex::new(None)),
        http,
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/api/state", get(get_state))
        .route("/api/run", post(run_cmd))
        .route("/api/config", get(get_config).post(post_config))
        .route("/api/spawn", post(spawn_process))
        .route("/api/processes", get(list_processes))
        .route("/api/kill", post(kill_process))
        .route("/api/opsec-quick", get(opsec_quick))
        .route("/api/sentinel/health", get(sentinel_health))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    println!("AXIS GUI v6.0 (Windows) → http://127.0.0.1:{port}");
    println!("  axis binary: {}", std::env::var("AXIS_BIN").unwrap_or_else(|_| "axis (from PATH)".into()));
    println!("  config dir:  {}", config_dir().display());
    println!("  sentinel engine: {SENTINEL_URL}  (suba sentinel_engine.py pra módulos sentinel-*)");
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}

async fn index() -> Html<&'static str> { Html(INDEX_HTML) }

async fn get_state(State(st): State<AppState>) -> Json<serde_json::Value> {
    let scope = st.scope.lock().await.clone();
    Json(serde_json::json!({
        "scope": scope,
        "findings": count_findings(),
        "hostname": hostname(),
        "version": "6.0.0",
        "os": "windows",
    }))
}

// ═══════════════════════════════════════════════════════════════════════
// SENTINEL PROXY
// ═══════════════════════════════════════════════════════════════════════
fn sentinel_action(args: &[String]) -> Option<String> {
    if args.len() < 3 { return None; }
    if args[0] != "defense" || args[1] != "--action" { return None; }
    let action = &args[2];
    if !action.starts_with("sentinel-") { return None; }
    Some(action["sentinel-".len()..].to_string())
}

async fn sentinel_health(State(st): State<AppState>) -> Json<serde_json::Value> {
    match st.http.get(format!("{SENTINEL_URL}/health"))
        .timeout(Duration::from_secs(2)).send().await
    {
        Ok(r) => match r.json::<serde_json::Value>().await {
            Ok(v) => Json(v),
            Err(_) => Json(serde_json::json!({"ok": false, "error": "parse"})),
        },
        Err(_) => Json(serde_json::json!({"ok": false, "error": "engine offline"})),
    }
}

async fn sentinel_call(st: &AppState, action: &str, args: &[String]) -> Json<RunResp> {
    let mut body = serde_json::Map::new();
    let mut it = args.iter().skip(3); // pula "defense", "--action", "sentinel-xxx"
    while let Some(k) = it.next() {
        if let Some(flag) = k.strip_prefix("--") {
            if let Some(v) = it.next() {
                body.insert(flag.to_string(), serde_json::Value::String(v.clone()));
            }
        }
    }
    let url = format!("{SENTINEL_URL}/{action}");
    match st.http.post(&url).json(&body).send().await {
        Ok(r) => {
            let code = r.status().as_u16() as i32;
            let txt = r.text().await.unwrap_or_default();
            Json(RunResp {
                stdout: txt,
                stderr: String::new(),
                code: if code == 200 { 0 } else { code },
                duration_ms: 0,
                blocked: None,
                needs_confirm: None,
            })
        }
        Err(e) => Json(RunResp {
            stdout: String::new(),
            stderr: format!("sentinel engine offline em {SENTINEL_URL}: {e}\n\nsuba com: python sentinel_engine.py --port 8899"),
            code: -1,
            duration_ms: 0,
            blocked: None,
            needs_confirm: None,
        }),
    }
}

// ═══════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════
fn config_dir() -> PathBuf {
    std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"))
        .map(|h| PathBuf::from(h).join(".axis"))
        .unwrap_or_else(|| PathBuf::from(".axis"))
}

fn read_env_file() -> HashMap<String, String> {
    let mut map = HashMap::new();
    let path = config_dir().join("env");
    if let Ok(raw) = std::fs::read_to_string(path) {
        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if let Some((k, v)) = line.split_once('=') {
                map.insert(k.trim().to_string(), v.trim().trim_matches('"').to_string());
            }
        }
    }
    map
}

fn write_env_file(map: &HashMap<String, String>) -> std::io::Result<()> {
    let dir = config_dir();
    let _ = std::fs::create_dir_all(&dir);
    let mut keys: Vec<_> = map.keys().collect();
    keys.sort();
    let mut out = String::new();
    for k in keys { out.push_str(&format!("{}={}\n", k, map[k])); }
    std::fs::write(dir.join("env"), out)
}

fn is_irreversible(args: &[String]) -> bool {
    if args.is_empty() { return false; }
    let sub = |n: usize| args.get(n).map(|s| s.as_str()).unwrap_or("");
    match sub(0) {
        "opsec" => matches!(
            (sub(1), sub(2)),
            ("mac", "set") | ("mac", "reset")
                | ("killswitch", "enable") | ("killswitch", "disable")
                | ("persist", "install")
        ),
        "persist" => true,
        "forensics" => matches!(sub(1), "disk" | "memory"),
        "defender"  => matches!(sub(1), "disable" | "exclusions"),
        "firewall"  => matches!(sub(1), "disable"),
        "bitlocker" => matches!(sub(1), "off" | "disable"),
        "bcdedit"   => true,
        "vssadmin"  => true,
        "wevtutil"  => matches!(sub(1), "cl" | "clear-log"),
        "schtasks"  => matches!(sub(1), "delete"),
        "reg"       => matches!(sub(1), "delete"),
        "sc"        => matches!(sub(1), "delete"),
        "hardening" => matches!(sub(2),
            "asr-enable" | "credguard-enable" | "smb-harden"
            | "firewall-default-deny" | "defender-aggressive" | "wdac-enforce"
        ),
        "evasion" => matches!(sub(1), "freeze-edr" | "hyperv-nat"),
        "emulate" => matches!(sub(1), "byovd-killer"),
        "defense" => matches!(
            (sub(1), sub(2)),
            ("--action", "sysmon-install")
                | ("--action", "wdac-enforce")
                | ("--action", "contain")
        ),
        _ => false,
    }
}

async fn run_cmd(State(st): State<AppState>, Json(req): Json<RunReq>) -> Json<RunResp> {
    // Roteia defense --action sentinel-* pro engine Python
    if let Some(action) = sentinel_action(&req.args) {
        return sentinel_call(&st, &action, &req.args).await;
    }

    if is_irreversible(&req.args) && req.confirm != Some(true) {
        return Json(RunResp {
            stdout: String::new(), stderr: String::new(), code: 0, duration_ms: 0,
            blocked: Some("irreversible operation — confirm required".into()),
            needs_confirm: Some(true),
        });
    }
    let start = Instant::now();
    let mut cmd = tokio::process::Command::new(&st.axis_bin);
    cmd.args(&req.args);
    for (k, v) in read_env_file() { cmd.env(k, v); }
    let out = cmd.output().await;
    match out {
        Ok(o) => Json(RunResp {
            stdout: String::from_utf8_lossy(&o.stdout).to_string(),
            stderr: String::from_utf8_lossy(&o.stderr).to_string(),
            code: o.status.code().unwrap_or(-1),
            duration_ms: start.elapsed().as_millis(),
            blocked: None, needs_confirm: None,
        }),
        Err(e) => Json(RunResp {
            stdout: String::new(),
            stderr: format!("spawn {}: {e}", st.axis_bin),
            code: -1, duration_ms: start.elapsed().as_millis(),
            blocked: None, needs_confirm: None,
        }),
    }
}

async fn get_config() -> Json<serde_json::Value> {
    let dir = config_dir();
    let raw_env = read_env_file();
    let mut vars = serde_json::Map::new();
    for (k, v) in raw_env {
        let sensitive = k.contains("TOKEN") || k.contains("KEY")
            || k.contains("PASS") || k.contains("WEBHOOK") || k.contains("SECRET");
        let masked = if sensitive && !v.is_empty() {
            format!("set ({} chars)", v.len())
        } else { v };
        vars.insert(k, serde_json::Value::String(masked));
    }
    let scope_yaml = std::fs::read_to_string(dir.join("scope.yaml")).unwrap_or_default();
    Json(serde_json::json!({
        "env": vars,
        "scope_yaml": scope_yaml,
        "env_path": dir.join("env").display().to_string(),
        "scope_path": dir.join("scope.yaml").display().to_string(),
        "axis_bin": std::env::var("AXIS_BIN").unwrap_or_else(|_| "axis".into()),
        "gui_port": std::env::var("AXIS_GUI_PORT").unwrap_or_else(|_| "7878".into()),
        "os": "windows",
    }))
}

async fn post_config(State(st): State<AppState>, Json(req): Json<ConfigReq>)
    -> Json<serde_json::Value>
{
    let dir = config_dir();
    let _ = std::fs::create_dir_all(&dir);
    if let Some(yaml) = req.scope_yaml {
        let _ = std::fs::write(dir.join("scope.yaml"), &yaml);
        *st.scope.lock().await = load_scope();
    }
    if let Some(new_env) = req.env {
        let mut map = read_env_file();
        for (k, v) in new_env {
            if v.is_empty() { map.remove(&k); } else { map.insert(k, v); }
        }
        let _ = write_env_file(&map);
    }
    Json(serde_json::json!({"ok": true}))
}

async fn spawn_process(State(st): State<AppState>, Json(req): Json<SpawnReq>)
    -> Json<serde_json::Value>
{
    let mut cmd = tokio::process::Command::new(&st.axis_bin);
    cmd.args(&req.args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    for (k, v) in read_env_file() { cmd.env(k, v); }
    match cmd.spawn() {
        Ok(child) => {
            let pid = child.id().unwrap_or(0);
            st.processes.lock().await.insert(req.name.clone(), pid);
            Json(serde_json::json!({"spawned": true, "name": req.name, "pid": pid}))
        }
        Err(e) => Json(serde_json::json!({"spawned": false, "error": e.to_string()})),
    }
}

async fn list_processes(State(st): State<AppState>) -> Json<serde_json::Value> {
    let procs = st.processes.lock().await;
    let arr: Vec<_> = procs.iter()
        .map(|(n, p)| serde_json::json!({"name": n, "pid": p})).collect();
    Json(serde_json::json!({"processes": arr}))
}

async fn kill_process(State(st): State<AppState>, Json(req): Json<KillReq>)
    -> Json<serde_json::Value>
{
    let mut procs = st.processes.lock().await;
    if let Some(pid) = procs.remove(&req.name) {
        let _ = std::process::Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"]).status();
        Json(serde_json::json!({"killed": true, "name": req.name}))
    } else {
        Json(serde_json::json!({"killed": false, "error": "not found"}))
    }
}

async fn opsec_quick(State(st): State<AppState>) -> Json<serde_json::Value> {
    {
        let cache = st.opsec_cache.lock().await;
        if let Some((ts, val)) = cache.as_ref() {
            if ts.elapsed().as_secs() < 10 { return Json(val.clone()); }
        }
    }
    let scope_ok = config_dir().join("scope.yaml").exists();
    let (mac_spoofed, vpn_up) =
        tokio::task::spawn_blocking(windows_opsec_probe).await.unwrap_or((false, false));
    let vulnerable = !(scope_ok && mac_spoofed && vpn_up);
    let val = serde_json::json!({
        "scope_ok": scope_ok,
        "mac_spoofed": mac_spoofed,
        "vpn_up": vpn_up,
        "vulnerable": vulnerable,
    });
    *st.opsec_cache.lock().await = Some((Instant::now(), val.clone()));
    Json(val)
}

fn windows_opsec_probe() -> (bool, bool) {
    let ps_mac = r#"
$found = $false
Get-ChildItem 'HKLM:\SYSTEM\CurrentControlSet\Control\Class\{4d36e972-e325-11ce-bfc1-08002be10318}' -ErrorAction SilentlyContinue |
  ForEach-Object {
    $na = (Get-ItemProperty $_.PSPath -Name NetworkAddress -ErrorAction SilentlyContinue).NetworkAddress
    if ($na) { $found = $true }
  }
if ($found) { 'spoofed' } else { 'clean' }
"#;
    let mac_spoofed = run_ps(ps_mac).map(|s| s.trim() == "spoofed").unwrap_or(false);

    let ps_vpn = r#"
$vpn = Get-NetAdapter -ErrorAction SilentlyContinue |
  Where-Object {
    $_.Status -eq 'Up' -and (
      $_.InterfaceDescription -match 'VPN|TAP|WireGuard|Wintun|OpenVPN|Tailscale|ZeroTier|SoftEther|SSTP|IKEv2|L2TP' -or
      $_.Name -match 'VPN|TAP|WireGuard|Wintun|OpenVPN|Tailscale|ZeroTier'
    )
  }
if ($vpn) { 'up' } else { 'down' }
"#;
    let vpn_up = run_ps(ps_vpn).map(|s| s.trim() == "up").unwrap_or(false);
    (mac_spoofed, vpn_up)
}

fn run_ps(script: &str) -> Option<String> {
    let out = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", script])
        .output().ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).to_string())
    } else { None }
}

fn load_scope() -> Option<ScopeState> {
    let path = config_dir().join("scope.yaml");
    let raw = std::fs::read_to_string(path).ok()?;
    let mut engagement = String::new();
    let mut targets = Vec::new();
    let mut window_start = String::new();
    let mut window_end = String::new();
    let mut in_targets = false;
    for line in raw.lines() {
        let t = line.trim();
        if let Some(v) = t.strip_prefix("engagement:") { engagement = v.trim().into(); }
        else if let Some(v) = t.strip_prefix("window_start:") { window_start = v.trim().into(); }
        else if let Some(v) = t.strip_prefix("window_end:") { window_end = v.trim().into(); }
        else if t.starts_with("targets:") { in_targets = true; }
        else if in_targets && t.starts_with("- host:") {
            targets.push(t.trim_start_matches("- host:").trim().to_string());
        }
    }
    Some(ScopeState { engagement, targets, window_start, window_end })
}

fn count_findings() -> usize {
    let dir = config_dir();
    let db = dir.join("findings.sqlite");
    if db.exists() {
        if let Ok(o) = Command::new("sqlite3").arg(&db).arg("SELECT COUNT(*) FROM findings;").output() {
            if o.status.success() {
                if let Ok(n) = String::from_utf8_lossy(&o.stdout).trim().parse() { return n; }
            }
        }
    }
    let json_path = dir.join("findings.json");
    if let Ok(raw) = std::fs::read_to_string(json_path) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) {
            if let Some(arr) = v.as_array() { return arr.len(); }
        }
        return raw.lines().filter(|l| !l.trim().is_empty()).count();
    }
    0
}

fn hostname() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .or_else(|_| {
            Command::new("hostname").output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .ok_or(())
        })
        .unwrap_or_else(|_| "localhost".into())
}
