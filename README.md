# AXIS v6

Red Team 🤝 Blue Team — porque quebrar coisas é fácil, difícil é explicar no relatório.

![version](https://img.shields.io/badge/version-6.0.0-00e5ff)
![platform](https://img.shields.io/badge/platform-Windows%2010%2F11-0078d4)
![rust](https://img.shields.io/badge/rust-1.75%2B-orange)

> **AXIS**: Um ambiente integrado de segurança ofensiva e defensiva para Windows.
>
> Tradução: Um lugar onde o Red Team faz perguntas inconvenientes e o Blue Team precisa ter respostas melhores que "não sei, chefe".

---

## 📌 O que diabos é o AXIS?

O AXIS é uma plataforma integrada de Red Team e Blue Team, desenvolvida para Windows, com interface web local e CLI independente.

Nada de depender de 47 runtimes, 18 serviços externos e uma invocação demoníaca para abrir o programa.

São três componentes principais:

1. **`axis-gui.exe`** — servidor HTTP local em `127.0.0.1:7878`, responsável por servir a interface SPA. HTML, CSS e JS ficam embutidos diretamente em um `const &str`. Nada de pasta `static/` ou arquivos perdidos na dimensão da build.
2. **`axis`** — o verdadeiro funcionário do mês. Binário separado que executa as operações solicitadas pela GUI. A interface organiza, o CLI trabalha.
3. **`%USERPROFILE%\.axis\`** — diretório de estado, onde ficam configurações, chaves, escopo autorizado, findings e informações necessárias para reversões.

> **Regra fundamental:** cada botão da interface corresponde a um comando real do CLI.

Ou seja, não existe botão mágico que resolve tudo enquanto o desenvolvedor finge que entende o próprio código.

A GUI não possui autoridade para aprovar operações que o servidor rejeitou. O AXIS não confia nem no próprio JavaScript. E sinceramente? Correto.

---

## 🔄 O que mudou na versão 6.0?

Foi realizado um rebranding das superfícies visíveis.

A funcionalidade foi mantida, mas alguns nomes receberam uma apresentação mais profissional.

| Antes (v5.x) | Agora (v6.0) |
|---|---|
| `axis stealer --action browsers` | `axis collect --action browsers` |
| `axis attack byovd-killer` | `axis emulate byovd-killer` |
| `axis wifi --action scan` | `axis wireless --action scan` |
| `axis ad --action enum` | `axis directory --action enum` |
| `hades-*` | `sentinel-*` |
| `hades_engine.py` | `sentinel_engine.py` |
| Credential Access | Credential Hygiene |
| Ataque | Emulação de Adversário |

Aparentemente, trocar os nomes não aumenta o desempenho, mas pelo menos agora o programa não parece ter sido desenvolvido por um vilão de desenho animado.

---

## 🏗️ Arquitetura

O AXIS trabalha com uma separação clara entre interface, execução e armazenamento.

```
┌──────────────────────────────────────────────────────────────┐
│                    BROWSER · 127.0.0.1:7878                  │
│                                                                │
│   Módulos │ CLI/AURA │ APIs │ Sistema │ Docs                  │
│                                                                │
└────────────────────────────┬───────────────────────────────────┘
                              │
                        HTTP / API
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│                 AXIS-GUI · RUST / AXUM                        │
│                                                                │
│  • is_irreversible() → validação server-side                  │
│  • opsec_quick()     → cache de 10 segundos                   │
│  • tokio::process    → gerenciamento do CLI                   │
│                                                                │
└────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│                     AXIS · CLI                                │
│                                                                │
│  recon · opsec · collect · defense · hardening · forensics    │
│                                                                │
└────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│                %USERPROFILE%\.axis\                           │
│                                                                │
│  env · scope.yaml · findings.sqlite · opsec                   │
│                                                                │
└──────────────────────────────────────────────────────────────┘
```

A regra de ouro:

> `axis-gui.exe` não executa operações que o `axis` não conhece.

Se a GUI fechar, o CLI continua existindo. Se o navegador travar, o backend não perde sua identidade. Se o CSS quebrar, bom... pelo menos o problema não é de segurança.

---

## 💻 Instalação

### Pré-requisitos

| Requisito | Versão | Observação |
|---|---|---|
| Windows 10 (1909+) ou 11 | — | Sistema suportado |
| PowerShell | 5.1+ | Já vem instalado |
| Rust | 1.75+ | Necessário para compilação |
| WireGuard | Opcional | Para funcionalidades de killswitch |
| sqlite3.exe | Opcional | Sem ele, fallback para JSON |

### Compilando

```powershell
git clone <https://github.com/delaccont-sketch/Axis-core>
cd axis\gui
cargo build --release
```

O resultado estará em:

```
target\release\axis-gui.exe
```

Também é necessário compilar o CLI separadamente e disponibilizar `axis.exe` no PATH ou configurar `AXIS_BIN`.

### Inicializando

```powershell
# Inicialização normal
.\target\release\axis-gui.exe

# Ou especificando o executável do CLI
$env:AXIS_BIN = "C:\axis\target\release\axis.exe"
.\target\release\axis-gui.exe
```

Agora abra:

```
http://127.0.0.1:7878
```

Pronto. A central de operações está aberta.

> Observação: não, isso não significa que você virou o protagonista de Mr. Robot. Ainda precisa entender o que está fazendo.

### ⚠️ Permissões administrativas

Alguns módulos exigem PowerShell elevado:

- Hardening
- Windows Defender
- Firewall
- Gerenciamento de MAC
- Aquisição de memória

Se o programa for iniciado como usuário comum, essas operações podem falhar com `Permission denied` ou código de saída `5`.

A GUI não solicita elevação automaticamente.

O Windows não vai entregar privilégios administrativos só porque você abriu uma interface bonita.

---

## 🚀 Quick Start

Antes de começar, configure o escopo autorizado.

Sem escopo, sem operação de rede. Simples assim.

```powershell
# 1. Criando um escopo de laboratório local
@"
engagement: lab-window
window_start: 2026-01-01
window_end:   2026-12-31
contact: voce@example.com
targets:
  - host: 127.0.0.1
"@ | Set-Content "$env:USERPROFILE\.axis\scope.yaml"

# 2. Inicializando a GUI
.\axis-gui.exe

# 3. Abrindo o console AURA
```

Dentro da aba CLI:

```
aura init
aura whoami
aura vision
```

Esses comandos apresentam a configuração inicial, a identidade do operador e o dashboard.

O `aura ritual` também está disponível para executar a sequência de auditoria e apresentar as opções de postura OPSEC.

---

## 🖥️ Conhecendo a interface

O AXIS possui cinco abas principais.

### 1. 📦 Módulos — onde o trabalho acontece

A central de operações.

- 19 categorias
- Aproximadamente 95 módulos
- Busca fuzzy
- Palette de comandos com `Ctrl + K`
- Formulários dinâmicos
- Output separado em `stdout`, `stderr` e `raw`
- Suporte a cores ANSI

A busca tolera erros de digitação. Porque até desenvolvedores profissionais digitam "recon" como se estivessem tendo uma convulsão no teclado.

**Confirmação de operações sensíveis:**

Operações classificadas como irreversíveis exigem uma confirmação explícita.

O modal solicita que o operador digite:

```
EXECUTE
```

A validação acontece tanto no cliente quanto no servidor.

Editar o JavaScript para tentar contornar a confirmação é basicamente tentar convencer uma porta fechada a virar uma janela.

### 2. ⌨️ CLI — console AURA

Um terminal dedicado com autocomplete contextual.

Suporta:

- Comandos base
- Família de comandos AURA
- Autocomplete
- Atalhos rápidos
- Execução de comandos disponíveis no CLI

É basicamente o terminal tradicional, mas com uma interface que não parece ter sido projetada durante a Revolução Industrial.

### 3. ⚡ APIs — integrações externas

Central de configuração de credenciais e serviços.

Integrações disponíveis:

| Serviço | Finalidade |
|---|---|
| HIBP | Inteligência sobre exposição de credenciais |
| Shodan | Inteligência de ativos conectados |
| VirusTotal | Análise de indicadores |
| GitHub | Integração com a plataforma |
| Telegram | Bot e chat |
| Discord | Webhook |
| Cloudflare Worker | Integração externa |
| Proxy | Configuração de proxy padrão |

Cada card possui:

- `BIND`: configura a credencial
- `TEST`: verifica a configuração
- Indicador de status

As chaves são mascaradas na leitura e não devem ser exibidas em respostas de status.

O objetivo é integrar serviços, não transformar seu token em decoração pública.

### 4. ⚙️ Sistema — central de configuração

Aqui ficam as configurações gerais:

- Porta do servidor
- Caminho do binário `axis`
- Processos em background
- Configuração do `scope.yaml`
- Estado OPSEC
- Configuração do SENTINEL

Basicamente, a sala de máquinas.

Se alguma coisa der errado, é um dos primeiros lugares para olhar.

### 5. 📚 Docs — documentação

Este README, incorporado à própria SPA.

Sim, a documentação mora dentro do programa.

É uma relação simbiótica entre código e documentação. Até o bug precisa ler o manual.

---

## ✨ AURA — Assistente de Operações

AURA é uma camada de orquestração sobre as rotas existentes da API.

Não adiciona endpoints novos. Apenas simplifica o acesso às funcionalidades.

Em outras palavras: menos digitação, mais organização e menos chances de esquecer o que estava fazendo.

### Comandos disponíveis

| Comando | Função |
|---|---|
| `aura` / `aura help` | Exibe o cartão de ajuda |
| `aura init` | Onboarding e configuração inicial |
| `aura bind <svc> <valor>` | Configura uma credencial |
| `aura unbanish <svc>` | Remove uma credencial |
| `aura whoami` | Exibe informações do operador |
| `aura vision` | Dashboard de host e OPSEC |
| `aura ping` | Executa auditoria OPSEC |
| `aura ritual` | Sequência de auditoria e opções OPSEC |
| `aura themes` | Lista os temas |
| `aura theme <nome>` | Altera o tema |
| `aura export` | Exporta configurações |
| `aura wipe` | Remove todas as chaves após confirmação |
| `aura crown` | Banner do operador |
| `aura lock` | Encerra a sessão |

### Temas disponíveis

```
axis
aurora
matrix
blood
ghost
```

Porque até uma ferramenta de segurança merece escolher sua skin.

### Aliases de credenciais

```
vt            → virustotal
cf            → cloudflare
telegram      → telegram-token
telegram_chat → telegram-chat
```

> **Nota de segurança:** `aura export` pode incluir credenciais em texto plano. Trate a saída como informação sensível.

---

## 🧩 Categorias de módulos

O AXIS reúne aproximadamente 95 módulos em 19 categorias.

Sim, é bastante coisa. O menu de configurações já está praticamente pedindo férias.

| Categoria | Especialidade | Exemplos |
|---|---|---|
| Branding & Utilidades | Identidade e utilidades | Banner, imagem para ASCII |
| OPSEC · Operador | Postura operacional | Auditoria, MAC, killswitch |
| Reconhecimento | Descoberta de ativos | DNS, fingerprint, inventário |
| OSINT · Inteligência Pública | Fontes abertas | WHOIS, HIBP, Shodan |
| Emulação de Adversário | Testes controlados | Enumeração, análise de exposição |
| Credential Hygiene | Higiene de credenciais | Inventário e rotação local |
| Defesa Ativa | Proteção do host | Defender, Firewall, YARA, EVTX |
| Purple Team | Validação defensiva | Atomic Red Team, Coverage Map |
| Hardening · Endurecimento | Fortalecimento do sistema | CIS Baseline, SMB, ASR |
| Emulação Avançada | Pesquisa em laboratório | Técnicas experimentais e validação |
| Detecção & Resposta | Investigação de eventos | Sysmon, ETW, playbooks |
| SENTINEL | Monitoramento modular | Cinco monitores e correlator |
| Cloud Security | Segurança em nuvem | AWS, Azure, GCP |
| Container Security | Segurança de containers | Docker, Kubernetes, RBAC |
| IoT · Embarcados | Sistemas embarcados | Firmware, MQTT |
| Hardware Físico | Pesquisa de hardware | Proxmark3, UART |
| Supply Chain | Integridade de dependências | Typosquatting, Confusion |
| Forense Digital | Investigação digital | Triage, memória, disco |
| Relatórios | Documentação técnica | Exportação Markdown |

### ⚖️ Balanço operacional

A superfície defensiva possui mais módulos que a ofensiva consolidada.

- Defesa Ativa
- Hardening
- Detecção & Resposta
- Purple Team

Juntas, representam aproximadamente 45 módulos.

A superfície de emulação e higiene de credenciais representa aproximadamente 15 módulos.

Os coletores também foram reorganizados em três famílias principais.

> **Filosofia do projeto:** detectar, compreender, validar e corrigir.

Porque descobrir uma vulnerabilidade e simplesmente deixá-la lá é o equivalente digital a encontrar uma infiltração e colocar um balde embaixo para sempre.

---

## 🛡️ OPSEC — Segurança operacional

O AXIS disponibiliza ferramentas e informações para auxiliar na postura operacional do host.

Mas atenção: nenhuma ferramenta garante anonimato ou proteção absoluta.

A segurança depende da configuração, do ambiente e das decisões do operador.

### 1. Killswitch

Integração com Windows Firewall e WireGuard.

O objetivo é controlar a conectividade do host de acordo com a interface VPN configurada.

Uma configuração incorreta pode interromper a conectividade legítima. Valide as regras e tenha um procedimento de recuperação antes de utilizar.

### 2. Gerenciamento de MAC

O AXIS mantém informações do endereço MAC original para permitir reversão de alterações.

Os recursos de gerenciamento devem ser utilizados apenas em adaptadores e equipamentos próprios ou autorizados.

### 3. Indicador de postura

A GUI consulta `/api/opsec-quick` antes de apresentar determinadas operações sensíveis.

O estado pode incluir:

- Disponibilidade da interface VPN
- Estado do adaptador
- Presença de escopo
- Indicadores de configuração OPSEC

A consulta utiliza cache de 10 segundos.

Se alguma condição não estiver adequada, a interface apresenta o estado identificado e as opções disponíveis.

> **Importante:** um indicador verde não significa que o host está invulnerável. Significa apenas que as verificações implementadas não identificaram os problemas avaliados.

O verde é um indicador, não uma bênção divina.

---

## 🎯 Gate de escopo

Uma das partes mais importantes do AXIS.

Todo módulo que interage com a rede precisa validar o arquivo:

```
%USERPROFILE%\.axis\scope.yaml
```

Sem escopo válido:

```
exit 3
```

Alvo não autorizado:

```
exit 3
```

O sistema não deve enviar pacotes para alvos fora do escopo aprovado.

### Exemplo de configuração

```yaml
engagement: acme-q4
window_start: 2026-11-01
window_end:   2026-11-30
contact: security@acme.example

targets:
  - host: 127.0.0.1
    ports: [22, 80, 443]
```

Utilize somente endereços e sistemas explicitamente autorizados.

O gate é aplicado pelo CLI, não apenas pela GUI.

Alterações realizadas diretamente no arquivo podem não atualizar o cache do servidor. Utilize a função **SALVAR ESCOPO** para invalidar a configuração armazenada.

O escopo é a fronteira do projeto. Não é uma sugestão decorativa em YAML.

---

## 🚨 Controle de operações sensíveis

O AXIS possui uma allowlist de operações classificadas como irreversíveis em `is_irreversible()`.

Essas operações exigem confirmação validada no servidor.

Exemplos de categorias protegidas:

| Categoria | Exemplos |
|---|---|
| OPSEC | Alterações de MAC e configurações de conectividade |
| Persistência | Alterações em tarefas persistentes |
| Forense | Aquisição de disco e memória |
| Defender | Alterações de proteção e exclusões |
| Firewall | Alterações de configuração |
| BitLocker | Alterações no estado de proteção |
| Sistema | Operações administrativas sensíveis |

A lista exata é definida no código-fonte.

**Por que isso existe?**

Porque um clique acidental não deveria ter o poder de transformar uma tarde tranquila em um incidente de recuperação do Windows.

---

## 📂 Estrutura de configuração

O diretório principal é:

```
%USERPROFILE%\.axis\
```

| Arquivo | Finalidade |
|---|---|
| `env` | Credenciais e variáveis |
| `scope.yaml` | Escopo autorizado |
| `findings.sqlite` | Banco de achados |
| `findings.json` | Fallback caso SQLite não esteja disponível |
| `opsec/mac-orig.json` | Informações para reversão de MAC |
| `env.example` | Modelo de configuração |

As credenciais são carregadas pelo processo `axis` durante cada inicialização.

Chaves sensíveis são mascaradas na resposta da API de configuração.

### Variáveis de ambiente

É possível sobrescrever configurações diretamente no processo:

```powershell
$env:AXIS_GUI_PORT = "8787"
$env:AXIS_BIN = "C:\axis\target\release\axis.exe"
```

Lembre-se: variáveis de ambiente também podem conter informações sensíveis.

---

## 🔧 Troubleshooting — quando o Windows decide participar

Nem todo problema é um ataque hacker.

Às vezes é só o Windows sendo Windows.

| Sintoma | Possível causa | Solução |
|---|---|---|
| `Failed to fetch` | Servidor desligado | Inicie `axis-gui.exe` |
| `confirm required` | Confirmação ausente | Utilize o modal de confirmação |
| `scope.yaml missing` | Escopo inexistente | Configure o escopo autorizado |
| `axis.exe not found` | Binário fora do PATH | Configure `AXIS_BIN` |
| Inventário vazio | Limitações de acesso do sistema | Verifique permissões e compatibilidade |
| Sessão expirada | Estado de sessão não persistido | Verifique a configuração de sessão |
| PowerShell bloqueado | Políticas administrativas | Consulte a GPO responsável |
| OPSEC indica VPN indisponível | Interface não reconhecida | Verifique o adaptador e sua configuração |
| Cache desatualizado | Cache de 10 segundos | Aguarde ou reinicie a GUI |
| Porta ocupada | Outro processo utilizando 7878 | Configure outra porta |

Se nada funcionar, respire.

Não delete o System32.

---

## 🔐 Postura de segurança da GUI

A interface possui algumas limitações importantes.

Conhecê-las faz parte de utilizar o AXIS corretamente.

**Endereço de escuta**

O servidor escuta exclusivamente em:

```
127.0.0.1:7878
```

Não utilize `0.0.0.0` como endereço de escuta em ambientes expostos.

**Autenticação**

O login visual utiliza `localStorage`.

Ele é cosmético e não representa autenticação real.

O controle efetivo depende das validações realizadas no CLI e no servidor.

**Credenciais**

As chaves ficam armazenadas em texto plano no arquivo `env`.

Recomenda-se proteger o diretório utilizando BitLocker, EFS e permissões adequadas do Windows.

**Exportação**

`aura export` pode imprimir credenciais em texto plano.

Evite salvar ou compartilhar essa saída em ambientes não confiáveis.

**Exposição de rede**

Não exponha diretamente a porta 7878 à internet.

O AXIS foi projetado para uso local.

Segurança não é um tema visual. É uma propriedade da arquitetura.

---

## 🧑‍💻 Desenvolvimento

Estrutura mínima:

```
axis-gui/
├── Cargo.toml
└── src/
    └── main.rs
```

O `main.rs` contém:

- `INDEX_HTML`
- Interface SPA
- HTML
- CSS
- JavaScript
- Servidor Axum

Não existem:

- `static/`
- `build.rs`
- `include_str!`

Tudo está embutido no próprio executável.

### Compilação

```powershell
cargo build --release
```

Para desenvolvimento com recompilação automática:

```powershell
cargo watch -x run
```

### Adicionando módulos

A integração exige:

1. Definir a entrada correspondente em `MODULES` na interface.
2. Implementar o subcomando correspondente no CLI.
3. Mapear os campos do formulário para argumentos.
4. Validar permissões, escopo e efeitos colaterais.
5. Testar em laboratório antes de disponibilizar.

A GUI não precisa conhecer toda a implementação interna do CLI.

Ela precisa saber quais parâmetros enviar e como apresentar o resultado.

Menos acoplamento, menos sofrimento e menos arquivos misteriosamente desaparecendo.

---

## ⚖️ Uso autorizado

O AXIS é uma ferramenta de segurança profissional e deve ser utilizado exclusivamente em ambientes próprios ou com autorização explícita.

As técnicas implementadas podem produzir efeitos reais em sistemas e redes.

O uso não autorizado pode configurar infrações legais, incluindo:

- Lei 12.737/2012 — Brasil
- Computer Fraud and Abuse Act — Estados Unidos
- Computer Misuse Act — Reino Unido

**Usos esperados**

- Laboratórios próprios
- Máquinas virtuais
- Redes de teste
- Sistemas com autorização escrita
- Engajamentos profissionais de pentest
- Red Team contratado
- Resposta a incidentes

O arquivo `AUTHORIZED_USE.md`, localizado na raiz do repositório, contém a declaração de uso autorizado.

O escopo precisa estar aprovado antes de qualquer operação de rede.

> Se você não tem autorização para testar o sistema, não teste.

Ser curioso é ótimo. Virar personagem de um processo judicial é uma péssima DLC.

---

## 📜 Licença

MIT License

Consulte o arquivo `LICENSE`.

Sem garantia, sem suporte formal e sem SLA.

O AXIS é disponibilizado como está.

Se funcionar, ótimo.

Se não funcionar, abra o debugger antes de abrir uma nova dimensão de problemas.

---

## 🧠 Filosofia AXIS

O AXIS não foi criado para ser apenas uma coleção de comandos com uma interface bonita.

A proposta é integrar:

- Segurança ofensiva controlada
- Defesa ativa
- Hardening
- Monitoramento
- Forense
- Inteligência pública
- Automação
- Governança de escopo

Tudo em um ambiente modular, local e independente.

O objetivo não é apenas encontrar problemas. É entender, validar, documentar e corrigir.

---

<sub>
AXIS v6.0.0<br>
Build: <code>6.0.0</code><br>
Última atualização: <code>2026-11-07</code><br>
Plataforma: Windows 10 (1909+) / Windows 11<br>
Arquitetura: x86_64 · MSVC<br>
Backend: Rust · Axum · Tokio<br>
Frontend: SPA embutida<br>
CLI: <code>axis</code><br>
Interface: <code>axis-gui.exe</code><br><br>
Made with Rust, paranoia saudável e uma quantidade questionável de café.
</sub>
```
