# Docker

## Installation auf Ubuntu Server 24.04 LTS

### Docker Engine installieren

Docker ist nicht standardmäßig in den apt-Paketquellen enthalten. Daher muss die Paketquelle zuerst hinzugefügt werden.

```bash
sudo apt update
sudo apt install ca-certificates curl gnupg
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo $VERSION_CODENAME) stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt update
sudo apt install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo systemctl enable --now docker
sudo usermod -aG docker $USER
# Abmelden und wieder anmelden, damit die Gruppenänderungen wirksam werden
```

### Installation prüfen

```bash
docker --version
docker run hello-world
```

---

## Häufige Docker-Befehle

| Befehl | Beschreibung |
|--------|-------------|
| `docker pull <image>` | Image von [Docker Hub](../glossar.md#docker-hub) herunterladen |
| `docker build -t <name> .` | Image aus einem [Dockerfile](../glossar.md#dockerfile) bauen |
| `docker run -d -p <host>:<container> <image>` | Container im Hintergrund starten |
| `docker ps` | Laufende Container anzeigen |
| `docker ps -a` | Alle Container anzeigen |
| `docker stop <container>` | Laufenden Container stoppen |
| `docker rm <container>` | Container entfernen |
| `docker rmi <image>` | Image entfernen |
| `docker logs <container>` | Container-Logs anzeigen |
| `docker exec -it <container> bash` | Shell in einem laufenden Container öffnen |
| `docker system prune` | Nicht verwendete Ressourcen aufräumen |

---

# Docker Compose

Wenn mehrere [Container](../glossar.md#container) verwendet werden, wird das manuelle Starten in der richtigen Reihenfolge schnell komplex und unübersichtlich. Genau dafür gibt es [Docker Compose](../glossar.md#docker-compose). Mit Docker Compose kannst du mehrere Container mit all ihren Optionen und Abhängigkeiten in einer einzigen Konfigurationsdatei definieren und dann mit einem einzigen Befehl starten. Das ist das Werkzeug, das du beim [Self-Hosting](../glossar.md#self-hosting) am häufigsten verwenden wirst.

---

Weitere Informationen in der offiziellen Dokumentation: [Docker](https://docs.docker.com/)
