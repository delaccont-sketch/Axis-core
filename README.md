# Axis-core

Plataforma integrada de **Red Team e Blue Team** para Windows. Interface web local + CLI.
Zero dependência de runtime além do binário `axis` e do próprio `axis-gui.exe`.

![version](https://img.shields.io/badge/version-6.0.0-00e5ff) ![platform](https://img.shields.io/badge/platform-Windows%2010%2F11-0078d4) ![rust](https://img.shields.io/badge/rust-1.75%2B-orange)

---

## O que é

AXIS reúne três superfícies num único servidor local:

1. **`axis-gui.exe`** — servidor HTTP em `127.0.0.1:7878` que serve uma SPA (todo o HTML/CSS/JS embutido em um `const &str`, sem `static/`, sem `include_str!`).
2. **`axis`** — binário separado que a GUI invoca como subprocesso. Toda ação real mora nele. A GUI só orquestra.
3. **`%USERPROFILE%\.axis\`** — diretório de estado: `env` (chaves), `scope.yaml` (alvos autorizados), `findings.sqlite` (achados), `opsec/mac-orig.json` (MAC original para reversão).

Cada clique na GUI mapeia **1:1** para um comando `axis <subcomando>` que você consegue reproduzir no PowerShell elevado. Nada acontece "por mágica" — o cliente não pode fazer nada que o servidor não tenha aprovado.

### Renomeações v6.0

Rebrand profissional das superfícies visíveis. Funcionalidade idêntica — só nomenclatura.

| v5.x | v6.0 |
|---|---|
| `axis stealer --action browsers` | `axis collect --action browsers` |
| `axis attack byovd-killer` | `axis emulate byovd-killer` |
| `axis wifi --action scan` | `axis wireless --action scan` |
| `axis ad --action enum` | `axis directory --action enum` |
| `defense --action hades-*` | `defense --action sentinel-*` |
| `hades_engine.py` | `sentinel_engine.py` |
| categoria "Credential Access" | categoria "Credential Hygiene" |
| categoria "Ataque" | categoria "Emulação de Adversário" |

---

## Arquitetura

```

┌──────────────────────────────────────────────────────────────┐
│  Browser (127.0.0.1:7878)                                    │
│  ┌────────────┬────────────┬────────────┬─────────┬───────┐  │
│  │  módulos   │ ⌨ cli/aura │ ⚡ apis    │⚙ sistema│ docs  │  │
│  └─────┬──────┴──────┬─────┴─────┬──────┴────┬────┴───┬───┘  │
└────────┼─────────────┼───────────┼───────────┼────────┼──────┘
│  fetch /api/run /api/config /api/spawn /api/state
▼
┌──────────────────────────────────────────────────────────────┐
│  axis-gui (Rust · axum · tokio)                              │
│  · is_irreversible()  → allowlist server-side                │
│  · opsec_quick()      → cache 10s · registry + Get-NetAdapter│
│  · tokio::process::Command → axis <args>                     │
└────────────────────────┬─────────────────────────────────────┘
▼
┌──────────────────────────────────────────────────────────────┐
│  axis (CLI · binário separado)                               │
│  recon · opsec · collect · defense · hardening · forensics…  │
└────────────────────────┬─────────────────────────────────────┘
▼
┌──────────────────────────────────────────────────────────────┐
│  %USERPROFILE%.axis\                                        │
│  env · scope.yaml · findings.sqlite · opsec\mac-orig.json    │
└──────────────────────────────────────────────────────────────┘

```

Regra de ouro: `axis-gui.exe` nunca executa nada que `axis` não saiba fazer. Se você matar a GUI, todos os comandos continuam vivos na CLI.

---

## Instalação (Windows)

### Pré-requisitos

| Requisito | Versão | Notas |
|---|---|---|
| Windows 10 (1909+) ou 11 | | |
| PowerShell | 5.1 (já vem) | |
| Rust toolchain | 1.75+ | `rustup target add x86_64-pc-windows-msvc` |
| WireGuard (opcional) | qualquer | necessário só se for usar killswitch |
| `sqlite3.exe` (opcional) | qualquer | se ausente, cai para `findings.json` |

### Build

```powershell
git clone <repo>
cd axis\gui
cargo build --release
```

Saída:

```
target\release\axis-gui.exe
```

Também é preciso compilar o CLI em axis\ (repo separado) e colocar axis.exe no PATH, ou apontar via AXIS_BIN.

Rodar

```powershell
# terminal normal
.\target\release\axis-gui.exe

# ou com binário explícito
$env:AXIS_BIN = "C:\axis\target\release\axis.exe"
.\target\release\axis-gui.exe
```

Abra http://127.0.0.1:7878.

Módulos de hardening, defender, firewall, MAC spoof e aquisição de memória exigem terminal elevado (Administrador). Se o axis-gui.exe rodar como usuário comum, esses módulos vão falhar com Permission denied ou exit 5 — a GUI não eleva sozinha.

---

Quick start

```powershell
# 1) escopo mínimo — sem isso, nada que toca a rede executa
@"
engagement: lab-window
window_start: 2026-01-01
window_end:   2026-12-31
contact: voce@example.com
targets:
  - host: 127.0.0.1
"@ | Set-Content "$env:USERPROFILE\.axis\scope.yaml"

# 2) subir a GUI
.\axis-gui.exe

# 3) abrir o navegador, autenticar (credenciais são locais — não há backend real),
#    depois abrir a aba ⌨ cli e rodar:
aura init          # mostra os slots de chave que faltam
aura ritual        # OPSEC completo: audit → killswitch → MAC
aura whoami        # cartão de identidade
```

---

Abas

módulos — o trabalho principal

Sidebar com 19 categorias, busca fuzzy tolerante a erro (Ctrl+K abre a palette). Form gerado dinamicamente; output com abas stdout / stderr / raw. Cores ANSI preservadas (truecolor, 256, 16).

Comandos com efeito destrutivo disparam modal vermelho exigindo digitar EXECUTE. O cliente e o servidor verificam independentemente — não adianta editar o JS.

⌨ cli — console AURA

Terminal dedicado com autocomplete contextual. Aceita todos os comandos base (set, get, run, audit, exfil) e a família aura. Chips de atalho abaixo do input para os comandos mais usados.

⚡ apis — chaves externas

Grid de cards: HIBP, Shodan, VirusTotal, GitHub, Telegram (token + chat), Discord webhook, Cloudflare Worker, proxy padrão. Cada card tem BIND (grava em env) e TEST. Status ✓ / ✗ reflete o env atual — mascarado na leitura, nunca ecoado pela rede.

⚙ sistema — configuração do programa

Porta, binário axis em uso, processos em background (spawn/kill), scope.yaml, cluster OPSEC (audit / killswitch / MAC / persistência) e controle do engine SENTINEL.

docs — este arquivo

Embutido na própria SPA. Não depende de internet.

---

Comandos AURA

Chamada AURA é uma camada de orquestração sobre as rotas existentes (/api/config, /api/run, /api/spawn, /api/state, /api/opsec-quick). Nenhum endpoint novo foi adicionado — só açúcar.

Comando Efeito
aura · aura help cartão de ajuda formatado
aura init onboarding: mostra slots em aberto com comandos prontos pra copiar
aura bind <svc> <valor> grava a chave mapeada — hibp, shodan, vt, github, telegram-token, telegram-chat, discord, cf, proxy
aura unbanish <svc> remove a chave
aura whoami operador, host, tema, escopo, findings
aura vision dashboard de duas colunas — host + OPSEC
aura ping roda axis opsec audit, imprime resultado colorido
aura ritual audit → oferece killswitch → oferece MAC, sequencial
aura themes · aura theme <nome> axis, aurora, matrix, blood, ghost
aura export dump JSON de env + scope.yaml + tema + operador
aura wipe apaga todas as chaves (com confirm() nativo)
aura crown banner de operador em caixa
aura lock encerra a sessão

Aliases aceitos no bind: vt = virustotal, cf = cloudflare, telegram = telegram-token, telegram_chat = telegram-chat.

---

Categorias de módulos

Total: ~95 módulos distribuídos em 19 categorias. A tabela completa está em docs na própria GUI. Resumo:

Categoria Foco Exemplos
Branding & Utilidades banner, imagem→ASCII 
OPSEC · Operador anti-forense do operador audit, MAC set/reset, killswitch, STUN leak, Tor NEWNYM
Reconhecimento descoberta port scan, banner grab, DNS enum, web fingerprint
OSINT · Inteligência Pública coleta pública dorks, whois, HIBP, Shodan
Emulação de Adversário exploração controlada wordlist, wireless scan/PMKID, privesc enum, persistência, directory bloodhound/kerberoast
Credential Hygiene inventário e rotação local navegadores, carteiras, apps, rotação, keepalive
Defesa Ativa detecção defender, firewall, autoruns, schtasks, services, patch level, bitlocker, credguard, SMB, YARA, EVTX query, persistence audit, LOLBAS, AMSI, ETW, WDAC, LAPS, ASR, conexões, procs assinados
Purple Team emulação controlada atomic red team, coverage map, caldera
Hardening · Endurecimento CIS baseline CIS baseline, ASR enable, Credential Guard, SMB, firewall default-deny, Defender agressivo
Emulação Avançada técnicas de evasão (lab) Moonwalk++, Tartarus Gate, EDR-Freeze, Hyper-V NAT, BYOVD, ADCS ESC, IPI/LLM
Detecção & Resposta caça e contenção Sysmon + ETW, sentinel de log, caça-injeção, guarda do dumper, WDAC, olho de hipervisor, playbook de contenção
SENTINEL · Defesa Ativa engine modular 5 monitores paralelos com correlator
Cloud Security multi-provider AWS (enum, S3, IMDS), Azure device code, GCP IMDS
Container Security escape + enum Docker, K8s RBAC, escape
IoT · Embarcados firmware + MQTT extração de firmware, subscribe
Hardware físico Proxmark3, UART scan
Supply Chain typosquat + confusion npm/pypi/crates
Forense Digital aquisição triage IR, memória (WinPmem), disco, timeline
Relatórios entrega gerar markdown

Balanço: a superfície defensiva (Defesa Ativa + Hardening + Detecção & Resposta + Purple = ~45 módulos) é maior que a ofensiva consolidada (Emulação + Credential Hygiene = ~15). Os ~40 coletores por alvo foram fundidos em 3 módulos por família (navegadores, carteiras, apps).

---

OPSEC

O AXIS não se auto-protege sozinho — ele expõe o estado e oferece as ferramentas. Três camadas, todas configuráveis pela aba ⚙ sistema ou CLI:

1. Killswitch (Windows Firewall + WireGuard)

Bloqueia todo tráfego de saída que não seja pela interface VPN. Se a VPN cair, o host fica offline — não vaza para o IP real.

```powershell
axis opsec killswitch enable --iface "WireGuard"
axis opsec killswitch --iface "WireGuard"   # → up:true, rules_present:true
axis opsec killswitch disable               # reversível
```

2. Rotação de MAC (registro NetworkAddress)

```powershell
# manual
axis opsec mac set "Wi-Fi"              # grava original em ~/.axis/opsec/mac-orig.json
axis opsec mac reset "Wi-Fi"            # restaura original

# daemon (não sobrevive reboot)
axis opsec mac schedule "Wi-Fi" --interval 300 --jitter 60

# persistente (Task Scheduler)
axis opsec persist install --iface "Wi-Fi" --interval 300 --jitter 60
```

O detect (/api/opsec-quick) lê HKLM\SYSTEM\CurrentControlSet\Control\Class{4d36e972-e325-11ce-bfc1-08002be10318}...\NetworkAddress em todos os adaptadores. Se algum tiver valor diferente do permanente, conta como spoofado.

3. Aviso de "postura" antes de ações sensíveis

Antes de qualquer módulo sensível ou com efeito destrutivo, a GUI consulta /api/opsec-quick (cache 10s). Se o host estiver com VPN down, MAC não-spoofado, ou sem scope.yaml, o modal mostra o estado atual e três opções: RODAR AUDIT, CONFIGURAR OPSEC, EXECUTAR (red).

Calibração recomendada:

```powershell
axis opsec killswitch enable --iface "WireGuard"
axis opsec mac set "Wi-Fi"
axis opsec audit    # score ≥ 80, gaps=[]
```

---

Gate de escopo

Todo módulo que toca a rede lê %USERPROFILE%.axis\scope.yaml antes de qualquer pacote sair. Sem arquivo → exit 3, nada é enviado. Com arquivo, alvo fora da lista targets: → também exit 3.

Formato mínimo:

```yaml
engagement: acme-q4
window_start: 2026-11-01
window_end:   2026-11-30
contact: security@acme.example
targets:
  - host: 10.10.10.10
    ports: [22,80,443]
  - host: 10.10.10.20
```

O gate é aplicado no CLI (axis), não na GUI. Não adianta editar scope.yaml a quente com a GUI aberta sem passar pelo botão SALVAR ESCOPO — o cache do lado do servidor é invalidado só via POST /api/config.

---

Controle de operações sensíveis

A allowlist is_irreversible() em src/main.rs rejeita sem confirm:true, independentemente do que o cliente enviou:

prefixo motivos
opsec mac set, mac reset, killswitch enable, killswitch disable, persist install 
persist qualquer
forensics disk, memory 
defender disable, exclusions 
firewall disable 
bitlocker off, disable 
bcdedit vssadmin wevtutil cl schtasks delete reg delete sc delete qualquer

Para novos verbos destrutivos, edite a função — é uma linha.

---

Configuração (%USERPROFILE%.axis)

arquivo conteúdo
env KEY=VALUE por linha, ordenado. Lido em cada spawn do axis. Mascarado em /api/config (chaves com TOKEN/KEY/PASS/WEBHOOK/SECRET no nome).
scope.yaml alvos autorizados. Formato acima.
findings.sqlite achados. Se sqlite3.exe estiver ausente, cai para findings.json (array JSON).
opsec/mac-orig.json MAC original por interface, para reversão.
env.example gerado na primeira execução, com todas as chaves possíveis comentadas.

Variáveis de ambiente do processo axis-gui.exe sobrescrevem o arquivo:

```powershell
$env:AXIS_GUI_PORT = "8787"
$env:AXIS_BIN      = "C:\axis\target\release\axis.exe"
```

---

Troubleshooting

Sintoma Causa Correção
fetch error: Failed to fetch servidor off iniciar .\axis-gui.exe, abrir http://127.0.0.1:7878
⚠ irreversible operation — confirm required cliente sem confirm:true digitar EXECUTE no modal
exit 3 · scope.yaml missing sem arquivo, ou alvo não listado criar via aba ⚙ sistema → Escopo
spawn axis: No such file axis.exe fora do PATH $env:AXIS_BIN = "..." ou $env:PATH += ";C:\axis\target\release"
credential inventory retorna vazio (Chrome/Edge) App-Bound Encryption 127+ usar Firefox, ou injetar via --remote-debugging-port
Sessão morre em 5min rotação não persistida cred-rotate ou keepalive ativo
PowerShell bloqueado por GPO ExecutionPolicy restritiva servidor chama com -ExecutionPolicy Bypass; se GPO bloquear, o probe cai silencioso e OPSEC fica ✗
opsec audit retorna ✗ vpn interface VPN não bate no regex ajustar Get-NetAdapter — regex em windows_opsec_probe (WireGuard, Wintun, TAP, OpenVPN, Tailscale, ZeroTier, SSTP, IKEv2, L2TP)
Cache de OPSEC preso em "vulnerável" cache 10s não expirou esperar ou reiniciar GUI
Porta 7878 ocupada outro processo $env:AXIS_GUI_PORT = "8787"

---

Postura de segurança da GUI

· Escuta somente 127.0.0.1 — não bind em 0.0.0.0. Se quiser acessar de outro host, use túnel SSH.
· Não tem autenticação real. O "login" é localStorage — cosmético. O gate verdadeiro está no axis (escopo + is_irreversible).
· Nunca exponha a porta 7878 numa interface pública. Se precisar, coloque um reverse proxy com mTLS na frente.
· Chaves ficam em env em texto plano no disco do usuário. BitLocker ou EFS em %USERPROFILE%.axis\ é recomendado.
· aura export imprime as chaves em texto plano no terminal. Não redirecione para arquivo num host compartilhado.

---

Desenvolvimento

Estrutura mínima:

```
axis-gui/
├── Cargo.toml
└── src/
    └── main.rs      ← INDEX_HTML (const &str) + servidor axum
```

Não há static/, não há build.rs, não há include_str!. Tudo o que o navegador consome está em INDEX_HTML. Editar HTML/CSS/JS = editar dentro do r##"..."## e rebuildar.

```powershell
cargo build --release
# ou para dev com reload
cargo watch -x run
```

Para adicionar um módulo: acrescente uma entrada em MODULES no JS, e o subcomando correspondente no CLI axis. A GUI não precisa saber o que o axis faz — só mapeia campos → argv.

---

Uso autorizado

AXIS é uma ferramenta de segurança profissional. O gate de escopo existe porque as técnicas aqui implementadas são reais e o efeito delas em sistemas de terceiros é crime na maioria das jurisdições (Lei 12.737/2012 no Brasil, CFAA nos EUA, Computer Misuse Act no UK).

Uso esperado:

· Laboratórios próprios — VMs, hosts locais, redes de teste.
· Sistemas com autorização escrita — contrato, NDA, ou regra de bug bounty que cubra a ação.
· Engajamentos contratados — pentest, red team, resposta a incidentes.

AUTHORIZED_USE.md na raiz do repo é a declaração vinculante. Cada módulo toca a rede só depois que o axis confirmou scope.yaml.

---

Licença

MIT — veja LICENSE. Sem garantia, sem suporte, sem SLA.

---

Build: 6.0.0 · Última atualização: 2026-11-07 · Host alvo: Windows 10 (1909+) / Windows 11 · x86_64 · MSVC