# Geheimnisse schützen - Spickzettel

## 1. Was sind "Geheimnisse" (Secrets)?

Secrets sind sensible Informationen, die deine Anwendung braucht, aber die niemand sonst sehen darf:

- Datenbankpasswörter
- API-Schlüssel (z.B. für Stripe, OpenAI, AWS)
- Verschlüsselungskeys
- Zugangsdaten für externe Dienste

Wenn diese Informationen in falsche Hände geraten, kann jemand in deine Datenbank einbrechen, deine API-Kosten explodieren lassen oder deine Daten stehlen.

**Windows-Analogie:** Stell dir vor, du schreibst dein Online-Banking-Passwort auf einen öffentlichen Zettel und hängst ihn ans schwarze Brett. Secrets im Code oder in Git zu lassen ist dasselbe.

## 2. .env-Dateien: Secrets aus dem Code heraushalten

Secrets gehören niemals direkt in den Code. Stattdessen nutzt man `.env`-Dateien (Environment-Dateien).

### Ohne .env (falsch):
```python
# config.py – so NICHT!
DB_PASSWORD = "mein_geheimes_passwort_123"
API_KEY = "sk-abc123xyz"
```

### Mit .env (richtig):
```bash
# .env – diese Datei bleibt auf dem Server, kommt nie in Git!
DB_PASSWORD=mein_geheimes_passwort_123
API_KEY=sk-abc123xyz
```

```python
# config.py – sauber, ohne Secrets
import os
DB_PASSWORD = os.getenv("DB_PASSWORD")
API_KEY = os.getenv("API_KEY")
```

Die `.env`-Datei liegt auf dem Server, der Code liegt in Git – Secrets und Code sind getrennt.

## 3. Secrets niemals in Git eincheckieren

Git merkt sich **jeden** Commit für immer. Selbst wenn du ein Secret später löschst, ist es in der Git-History noch vorhanden und kann von jedem gefunden werden, der Zugriff auf das Repository hat.

### Die .gitignore-Datei

Sage Git, welche Dateien es ignorieren soll:

```bash
nano .gitignore
```

Inhalt:
```
# Secrets und lokale Konfiguration
.env
.env.local
.env.production
*.key
*.pem
secrets.json
```

Prüfen, ob die .env-Datei tatsächlich ignoriert wird:
```bash
git status
```

Die `.env`-Datei sollte nicht in der Ausgabe erscheinen. Wenn sie es tut, wird sie noch nicht ignoriert.

### Was tun, wenn du ein Secret versehentlich commitet hast?

1. Das Secret sofort widerrufen (API-Key löschen, Passwort ändern)
2. Ein neues Secret generieren
3. Das Secret aus der Git-History entfernen (technisch aufwendig)
4. Als Faustregel: Sobald ein Secret in Git war, gehe davon aus, dass es kompromittiert ist

## 4. Dateiberechtigungen für Secrets

Selbst wenn eine Datei nur auf dem Server liegt, sollte sie nur für den Eigentümer lesbar sein.

Berechtigungen setzen:
```bash
chmod 600 .env
```

Was bedeutet 600?
- 6 = rw- (Eigentümer kann lesen und schreiben)
- 0 = --- (Gruppe hat keinen Zugriff)
- 0 = --- (Andere haben keinen Zugriff)

Prüfen:
```bash
ls -la .env
```

Ausgabe sollte so aussehen:
```
-rw------- 1 alice alice 256 Mar  1 12:00 .env
```

Eigentümer korrekt setzen:
```bash
chown alice:alice .env
```

## 5. Secrets in Docker: Umgebungsvariablen

In Docker-Containern übergibt man Secrets über [Umgebungsvariablen](../glossar.md#environment-variable-umgebungsvariable) – nicht über Dateien im Container.

### Methode 1: env_file in docker-compose.yml

```yaml
# docker-compose.yml
services:
  meine-app:
    image: meine-app:latest
    env_file:
      - .env
```

Die `.env`-Datei liegt auf dem Host-Server. Docker liest sie beim Start und übergibt die Werte an den Container – aber die Datei selbst ist nicht im Container-Image.

### Methode 2: Einzelne Variablen direkt angeben

```yaml
# docker-compose.yml
services:
  meine-app:
    image: meine-app:latest
    environment:
      - DB_HOST=localhost
      - DB_PORT=5432
      # Passwörter lieber per env_file, nicht hier direkt!
```

### Was du NICHT tun solltest:

```dockerfile
# Dockerfile – so NICHT!
ENV DB_PASSWORD=mein_geheimes_passwort
```

Alles in einem Dockerfile landet im Container-Image und ist für jeden sichtbar, der Zugriff auf das Image hat.

## 6. Eine .env.example-Datei bereitstellen

Damit andere Entwickler wissen, welche Variablen benötigt werden (ohne die echten Werte zu sehen), erstelle eine Beispieldatei:

```bash
# .env.example – DIESE Datei kommt in Git!
DB_PASSWORD=HIER_PASSWORT_EINTRAGEN
API_KEY=HIER_API_KEY_EINTRAGEN
DB_HOST=localhost
DB_PORT=5432
```

In die `.gitignore` kommt nur `.env`, nicht `.env.example`:
```
.env
```

So können neue Entwickler:
```bash
cp .env.example .env
# Dann .env mit echten Werten befüllen
```

## 7. Kurzer Überblick: Secret-Management-Konzepte

Für einfache Projekte reichen `.env`-Dateien völlig aus. Wenn du aber viele Server, ein Team oder sehr sensible Daten hast, gibt es professionellere Werkzeuge:

| Tool | Wofür |
|------|-------|
| **Bitwarden Secrets Manager** | Open-Source, gut für kleine Teams |
| **HashiCorp Vault** | Leistungsstark, für komplexe Infrastrukturen |
| **AWS Secrets Manager** | Wenn du ohnehin AWS nutzt |
| **Docker Secrets** | Für Docker Swarm-Umgebungen |

Für den Einstieg beim Self-Hosting ist eine gut geschützte `.env`-Datei mit `chmod 600` ein solider Start.

## 8. Checkliste: Secrets richtig schützen

- [ ] Keine Passwörter oder API-Keys direkt im Code
- [ ] `.env` in `.gitignore` eingetragen
- [ ] `.env`-Datei mit `chmod 600` geschützt
- [ ] `.env.example` ohne echte Werte in Git
- [ ] In Docker: `env_file` statt `ENV` im Dockerfile
- [ ] Nach versehentlichem Commit: Secret sofort widerrufen

## 9. Häufige Fehler

**Problem:** `.env` erscheint trotz Eintrag in `.gitignore` in `git status`.
**Lösung:** Die Datei ist möglicherweise schon getrackt. Git aus dem Tracking entfernen:
```bash
git rm --cached .env
```
Danach ist sie zwar noch auf der Festplatte, aber Git ignoriert sie.

**Problem:** Docker kann die `.env`-Datei nicht finden.
**Lösung:** Die `.env`-Datei muss im gleichen Verzeichnis wie die `docker-compose.yml` liegen. Pfad prüfen:
```bash
ls -la
```

**Problem:** Du weißt nicht mehr, welche Variablen deine Anwendung braucht.
**Lösung:** Das ist der Grund für `.env.example`. Immer pflegen, wenn neue Variablen hinzukommen.
