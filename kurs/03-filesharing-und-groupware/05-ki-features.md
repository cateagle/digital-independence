# KI-Features & KI-Backend

## Nextcloud und KI: Die "Smart"-Features

Nextcloud hat in den letzten Jahren intensiv KI-Features integriert.
Das Projekt heißt intern **Nextcloud AI** und umfasst:

| Feature               | Was es tut                                               |
|-----------------------|----------------------------------------------------------|
| Smart Search          | Volltextsuche über Dateiinhalt, nicht nur Dateinamen     |
| Text-to-Image         | Bilder aus Textbeschreibungen generieren                 |
| AI Text Generation    | Textvorschläge, Zusammenfassungen, Übersetzungen         |
| Transcript (Talk)     | Sprache-zu-Text für Aufnahmen in Nextcloud Talk          |
| Smart Inbox (Mail)    | Priorisierung von E-Mails                                |
| Recognize             | Automatisches Tagging von Fotos (Gesichter, Orte, etc.)  |

---

## Wo kommt die KI her? Das Backend-Problem

Nextcloud selbst trainiert und betreibt keine KI-Modelle. Die KI-Features brauchen
ein **externes Backend**, das die eigentliche Arbeit übernimmt.

Drei Optionen:

### 1. Nextcloud AI (Cloud-Service)
Nextcloud GmbH bietet den Zugang zu KI-Servern als Subscription an.
- Einfach, kein eigener Server nötig
- Kosten pro Nutzung
- Daten verlassen die eigene Infrastruktur → Datenschutz-Frage

### 2. Nextcloud Partner
- Einige der Premium Partner bieten auch direkt Nextcloud mit KI-Integration an.

### 2. OpenAI / externe APIs
Nextcloud kann gegen die OpenAI-API konfiguriert werden.
- Einfach zu konfigurieren
- Kosten pro API-Call
- Daten gehen zu OpenAI → keine echte [Datensouveränität](../../wiki/glossar.md#datensouveränität)
- Es gibt auch andere Dienstleister, die eine OpenAI-kompatible Schnittstelle bieten.

### 3. Lokales KI-Backend (Self-hosted)
Eigene KI-Modelle auf dem eigenen Server. Das ist der datenschutzfreundliche Weg.

---

## Exkurs: Ollama als lokales KI-Backend

[Ollama](https://ollama.com/) ist ein Tool, das es einfach macht, **Large Language Models ([LLMs](../../wiki/glossar.md#llm-large-language-model)) lokal zu betreiben**. Es läuft als Server, der eine OpenAI-kompatible API anbietet. Nextcloud kann also Ollama ansprechen, als wäre es OpenAI.

### Was Ollama macht

```
Nextcloud (App)
      ↓  HTTP-Request (OpenAI-kompatibl)
   Ollama
      ↓  lädt Modell in RAM
   LLM-Modell (z.B. Llama 3, Mistral, Phi-3)
      ↓
   Antwort zurück an Nextcloud
```

Ollama verwaltet Modelle wie [Docker](../../wiki/glossar.md#docker) Images verwaltet: Herunterladen, versionieren, tauschen.

### Ollama als Docker Container

Im Verzeichnis `~/ollama` eine `docker-compose.yml` anlegen:

```yaml
services:
  ollama:
    image: ollama/ollama
    container_name: ollama
    restart: unless-stopped
    ports:
      - "11434:11434"
    volumes:
      - ollama:/root/.ollama
    environment:
      OLLAMA_CONTEXT_LENGTH: 8192

volumes:
  ollama:
```

```bash
# Starten
docker compose up -d

# Modell herunterladen (Beispiel: Llama 3.2 3B – klein, läuft auch ohne GPU)
docker compose exec ollama ollama pull llama3.2:3b

# Testen
docker compose exec ollama ollama run llama3.2:3b "Erkläre Docker in einem Satz."
```

> **Kontextfenster:** Der Standardwert von 4096 Tokens ist oft zu klein. Sobald eine
> Konversation das Limit überschreitet, wird der älteste Teil still abgeschnitten –
> das Modell "vergisst" den Anfang und fängt an, inkonsistent zu antworten.
> `OLLAMA_CONTEXT_LENGTH: 8192` verdoppelt das Fenster. Größere Werte (16384, 32768)
> sind möglich, brauchen aber proportional mehr RAM.

#### GPU-Beschleunigung

Ohne GPU läuft Ollama auf der CPU – langsam, aber funktional. Mit GPU ist es 10–50× schneller.
Der nötige Compose-Eintrag hängt vom Hersteller ab:

**NVIDIA** (Voraussetzung: `nvidia-container-toolkit` auf dem Host installiert):

```yaml
services:
  ollama:
    image: ollama/ollama
    # ... (restliche Konfiguration wie oben)
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all
              capabilities: [gpu]
```

**AMD** (ROCm, nur Linux):

```yaml
services:
  ollama:
    image: ollama/ollama:rocm    # anderes Image-Tag!
    # ... (restliche Konfiguration wie oben)
    devices:
      - /dev/kfd
      - /dev/dri
```

**Intel Arc / integrierte GPU:**

```yaml
services:
  ollama:
    image: ollama/ollama
    # ... (restliche Konfiguration wie oben)
    devices:
      - /dev/dri/card0
      - /dev/dri/renderD128
```

> Für unsere VM im Kurs ist GPU-Durchleitung nicht relevant – VirtualBox kann keine
> GPU an Linux-Gäste weitergeben. Diese Konfigurationen sind für echte Hardware gedacht.

> **Hardware-Realität:** LLMs brauchen RAM. Viel RAM.
> - 3B-Parameter-Modell: ~3–4 GB RAM
> - 7B-Parameter-Modell: ~8 GB RAM
> - 13B-Parameter-Modell: ~16 GB RAM
>
> Ohne GPU läuft alles auf der CPU. Das ist langsam, aber funktioniert.
> Für eine VM im Kurs: Llama 3.2 3B oder Phi-3 Mini sind realistisch.

### Modelle im Überblick

| Modell          | Größe    | Stärken                             |
|-----------------|----------|-------------------------------------|
| llama3.2:3b     | ~2 GB    | Schnell, gut für einfache Tasks     |
| phi3:mini       | ~2.3 GB  | Microsoft, gut für (einfachen) Code |
| mistral:7b      | ~4 GB    | Ausgewogen, gut für Deutsch         |
| llama3:8b       | ~5 GB    | Gute Qualität, braucht mehr RAM     |
| deepseek-r1:8b  | ~5 GB    | Gut für Reasoning/Analyse           |

Erwartet nicht, dass diese Modelle gleiche qualität bieten wie die von OpenAI, Google, Anthropic, usw.. Die Modelle, die man aktuell in der Cloud bekommt, brauchen ein vielfaches an RAM/VRAM. Für viele Aufgaben braucht man das aber auch nicht umbedingt.

### Nextcloud mit Ollama verbinden

1. In Nextcloud: Admin → KI-Dienste (oder "Externe Dienste")
2. Ollama-URL eintragen: `http://localhost:11434` (oder die Container-IP)
3. Modell auswählen
4. Testen: In einem Textdokument die KI-Funktionen aufrufen

> **Netzwerk-Hinweis:** Nextcloud läuft im Docker-Container, Ollama ebenfalls.
> `localhost` innerhalb des Nextcloud-Containers zeigt nicht auf den Host.
> Die richtige Adresse ist die **Docker-Bridge-IP** des Hosts (`172.17.0.1`)
> oder beide Container müssen im selben Docker-Netzwerk sein.

---

## Warum lokale KI relevant ist

Die Diskussion über KI-Features in Nextcloud ist ein Mikrokosmos der größeren
Datensouveränitätsfrage:

- Cloud-KI (OpenAI, Anthropic, Google) ist einfacher – aber die Daten verlassen die eigene Infrastruktur
- Lokale KI (Ollama + offene Modelle) ist aufwändiger, aber vollständig unter eigener Kontrolle
- Die Qualität lokaler Modelle nähert sich Cloud-Modellen an, besonders für einfache Aufgaben

Für einen Self-Hoster, der Nextcloud gerade wegen der Datensouveränität betreibt, ist die Antwort auf die Frage "welches KI-Backend?" eigentlich klar.

---

## Was wir in der VM nicht schaffen

In unserer VM mit 4 GB RAM und CPU-only ist das KI-Backend nur eingeschränkt nutzbar:
- Ollama lässt sich starten
- Kleine Modelle (3B) funktionieren, sind aber langsam
- Die Integration in Nextcloud können wir demonstrieren

Wer das ernsthaft nutzen will, braucht echte Hardware oder einen Server mit GPU. Privat kann ein Gaming PC mit einer Grafikkarte mit mindestens 8 GB Grafikspeicher ausreichen. Besser sind jedoch 12-24 GB. Je nach Nutzungsverhalten und Anforderungen kann das tatsächlich auch für kleine Organisationen ausreichen. Wenn ihr mehr dazu wissen wollt, dann gibt es im Opencampus.sh / coding.waterkant Netzwerk einiges and Informationen und Personen, die sich damit auskennen.

Als generellen Tipp kann ich aber nur empfehlen, nicht einfach immer ds stärkste Modell zu nehmen. Es gibt Modelle, die nur ein hundertstel der Kosten/Hardware/Energie fressen wie die Flagschiffmodelle dier großen Anbieter. Wenn diese ausreichen, dann könnt ihr einfacher wechseln oder etwas selbst hosten.

---
