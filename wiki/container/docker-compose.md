# Docker Compose

## Was ist Docker Compose?

[Docker Compose](../glossar.md#docker-compose) ist ein Werkzeug zum Definieren und Ausführen von Multi-Container-Docker-Anwendungen. Du verwendest eine [YAML](../glossar.md#yaml-yet-another-markup-language)-Datei, um die Dienste, Netzwerke und Volumes deiner Anwendung zu konfigurieren, und startest dann alles mit einem einzigen Befehl.

---

## Grundstruktur einer docker-compose.yml

```yaml
version: '3.8'  # Compose file format version
services:
  servicename:
    image: <image>
    build: <build context>
    command: <override default command>
    environment:
      - VAR=value
    env_file:
      - .env
    ports:
      - "host:container"
    volumes:
      - ./host/path:/container/path
    depends_on:
      - other_service
    restart: always
    networks:
      - mynetwork
    user: "1000:1000"
    entrypoint: ["executable", "param1"]
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost"]
      interval: 30s
      timeout: 10s
      retries: 3
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
networks:
  mynetwork:
    driver: bridge
volumes:
  mydata:
```

---

## Häufige Dienstoptionen

- **image:** Name des zu verwendenden Images (z.B. `nginx:latest`).
- **build:** [Build-Kontext](../glossar.md#build-kontext) oder Objekt (z.B. `build: .` oder `build: { context: ./app, dockerfile: Dockerfile.dev }`).
- **command:** Standard-Befehl überschreiben.
- **environment:** [Umgebungsvariablen](../glossar.md#environment-variable-umgebungsvariable) setzen (Liste oder Dictionary).
- **env_file:** Umgebungsvariablen aus Datei(en) laden.
- **ports:** Host:Container-Ports zuordnen (z.B. `- "8080:80"`).
- **volumes:** Host-Pfade oder benannte [Volumes](../glossar.md#volume-docker) einbinden.
- **depends_on:** Startreihenfolge der Dienste festlegen.
- **restart:** Neustart-Richtlinie (`no`, `always`, `on-failure`, `unless-stopped`).
- **networks:** An benutzerdefinierte Netzwerke anhängen.
- **user:** Als bestimmten Benutzer/Gruppe ausführen.
- **entrypoint:** Standard-Entrypoint überschreiben.
- **healthcheck:** [Gesundheitsprüfungen](../glossar.md#health-check-gesundheitsprüfung) konfigurieren.
- **logging:** Logging-Treiber und Optionen konfigurieren.

---

## Beispiel: Multi-Dienst Compose-Datei

```yaml
version: '3.8'
services:
  web:
    image: nginx:alpine
    ports:
      - "8080:80"
    volumes:
      - ./html:/usr/share/nginx/html:ro
    depends_on:
      - app
  app:
    build: ./app
    environment:
      - APP_ENV=production
    volumes:
      - ./app:/app
    command: python app.py
    restart: unless-stopped
```

---

## Nützliche Compose-CLI-Befehle

| Befehl | Beschreibung |
|--------|-------------|
| `docker compose up -d` | Alle Dienste im Hintergrund starten |
| `docker compose down` | Alle Dienste, Netzwerke und Volumes aus der Datei stoppen und entfernen |
| `docker compose ps` | Laufende Dienste anzeigen |
| `docker compose logs` | Logs aller Dienste anzeigen |
| `docker compose build` | Dienste bauen oder neu bauen |
| `docker compose restart` | Dienste neu starten |
| `docker compose exec <service> <cmd>` | Befehl in einem laufenden Dienst ausführen |
| `docker compose stop` | Dienste stoppen |
| `docker compose start` | Gestoppte Dienste starten |
| `docker compose pull` | Dienst-Images herunterladen |
| `docker compose config` | Compose-Datei validieren und anzeigen |

---

## Best Practices

- `.env`-Dateien für Umgebungsvariablen und Geheimnisse verwenden (niemals sensible Daten direkt in die Compose-Datei schreiben).
- Benannte Volumes für persistente und zwischen Containern teilbare Daten verwenden.
- `depends_on` für grundlegende Startreihenfolge nutzen; für echte Bereitschaftsprüfung Healthchecks und Wait-for-it-Skripte implementieren.
- `docker-compose.yml`-Dateien unter Versionskontrolle (z.B. Git) halten und Anpassungen dokumentieren.
- Minimale Basis-Images und mehrstufige Builds für kleinere, sicherere Images verwenden.
- Container nicht als Root ausführen; wo möglich einen Nicht-Root-Benutzer angeben.
- Explizite Image-Tags (nicht `latest`) für reproduzierbare Builds verwenden.
- Produktions- und Entwicklungskonfigurationen mit mehreren Compose-Dateien oder Override-Dateien trennen.
- Nicht verwendete Ressourcen regelmäßig mit `docker system prune` und `docker volume prune` bereinigen.
- Compose v2 verwendet `docker compose` (mit Leerzeichen), nicht `docker-compose` (mit Bindestrich).

---

## Häufige Fehler und Lösungen

### 1. `latest`-Image-Tags verwenden
**Problem:** Builds können bei Änderungen am Upstream-Image brechen oder sich anders verhalten.
**Lösung:** Immer explizite Versions-Tags verwenden (z.B. `nginx:1.25.3`) für reproduzierbare Builds.

### 2. Geheimnisse direkt in Compose-Dateien schreiben
**Problem:** Passwörter und API-Schlüssel landen in der Versionskontrolle.
**Lösung:** Umgebungsvariablen, `.env`-Dateien (nicht committen) oder Docker Secrets für sensible Daten verwenden.

### 3. Keine benannten Volumes für persistente Daten
**Problem:** Daten gehen verloren, wenn Container entfernt werden.
**Lösung:** Benannte Volumes in der Compose-Datei für Datenbanken und wichtige Daten definieren und verwenden.

### 4. Container können nicht kommunizieren wegen falscher Ports
**Problem:** Dienste können sich nicht verbinden wegen falscher Port-Zuordnung oder fehlendem `depends_on`.
**Lösung:** `ports:` und `depends_on:` prüfen. Mit `docker compose ps` Zuordnungen verifizieren. Auch prüfen, ob die Ports nicht bereits von etwas anderem belegt sind.

### 5. Dateiberechtigungsprobleme zwischen Host und Container
**Problem:** Eingebundene Dateien/Ordner sind wegen Benutzer-Mismatch nicht erreichbar.
**Lösung:** Option `user:` im Dienst setzen oder Berechtigungen auf dem Host anpassen. Kompatible [UID/GID](../glossar.md#uidgid-user-id--group-id) verwenden.

### 6. Syntaxfehler in der Compose-Datei
**Problem:** YAML ist leerzeichen-sensitiv; Tabs oder falsche Einrückungen führen zu Fehlern.
**Lösung:** Leerzeichen (keine Tabs) verwenden, mit `docker compose config` validieren und ggf. einen YAML-Linter einsetzen.

### 7. Nicht verwendete Ressourcen nicht bereinigen
**Problem:** Speicherplatz wird durch ungenutzte Images, Container und Volumes verbraucht.
**Lösung:** Regelmäßig `docker system prune` und `docker volume prune` ausführen. Vorsicht: keine Versionen löschen, die noch als Backup benötigt werden.

---

## Weitere Beispiele & Inspiration

- Praxisnahe Docker Compose Setups unter [awesome-docker-compose.com](https://awesome-docker-compose.com/)

---

## Offizielle Dokumentation

- [Compose file reference](https://docs.docker.com/compose/compose-file/)
- [Compose CLI reference](https://docs.docker.com/engine/reference/commandline/compose/)
