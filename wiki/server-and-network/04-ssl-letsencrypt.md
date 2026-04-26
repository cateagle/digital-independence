# TLS/SSL und Let's Encrypt - Spickzettel

## 1. Was ist TLS/SSL?

[TLS](../glossar.md#tls-transport-layer-security) (Transport Layer Security) – oft noch [SSL](../glossar.md#ssl-secure-sockets-layer) genannt, obwohl SSL veraltet ist – ist das Protokoll, das die Verbindung zwischen Browser und Server verschlüsselt. Das ist der Unterschied zwischen `http://` und `https://`.

**Ohne TLS:**
```
Browser  ───── Klartext ─────► Server
         "Passwort: geheim123"
         (jeder im Netzwerk kann mitlesen!)
```

**Mit TLS:**
```
Browser  ─── verschlüsselt ──► Server
         "x7$#@kLm9..."
         (für Dritte nicht lesbar)
```

**Windows-Analogie:** Wie ein versiegelter Brief statt einer Postkarte. Jeder kann die Postkarte lesen – den versiegelten Brief nur der Empfänger.

TLS bietet:
- **Verschlüsselung:** Niemand kann den Inhalt mitlesen
- **Authentizität:** Du weißt, dass du wirklich mit dem richtigen Server sprichst (nicht mit einem Angreifer)
- **Integrität:** Die Daten wurden unterwegs nicht verändert

## 2. Was ist ein TLS-Zertifikat?

Ein TLS-Zertifikat ist eine digitale Bescheinigung, die beweist: "Dieser Server gehört wirklich zu dieser Domain."

Das Zertifikat enthält:
- Den Domainnamen (z.B. `example.com`)
- Den öffentlichen Schlüssel des Servers
- Gültigkeitsdauer
- Die Signatur einer [Zertifizierungsstelle](../glossar.md#ca-certificate-authority) (CA)

**Zertifizierungsstellen (CA):** Vertrauenswürdige Organisationen, die Zertifikate ausstellen und deren Echtheit bestätigen. Dein Browser vertraut einer Liste bekannter CAs – wenn eine CA ein Zertifikat signiert hat, akzeptiert der Browser es.

## 3. Was ist Let's Encrypt?

[Let's Encrypt](../glossar.md#lets-encrypt) ist eine kostenlose, automatisierte Zertifizierungsstelle. Sie stellt TLS-Zertifikate kostenlos aus, die von allen modernen Browsern anerkannt werden.

Vor Let's Encrypt kosteten Zertifikate oft 50–200 € pro Jahr. Let's Encrypt hat HTTPS für alle zugänglich gemacht.

Eigenschaften:
- Kostenlos
- Zertifikate gelten 90 Tage (werden automatisch erneuert)
- Vollständig automatisierbar über `certbot`

## 4. certbot installieren und Zertifikat ausstellen

`certbot` ist das offizielle Tool zum Beantragen und Erneuern von Let's Encrypt-Zertifikaten.

**Installation auf Ubuntu 24.04:**
```bash
sudo apt update
sudo apt install certbot python3-certbot-nginx
```

**Zertifikat für eine Domain ausstellen (mit automatischer Nginx-Konfiguration):**
```bash
sudo certbot --nginx -d example.com -d www.example.com
```

certbot wird:
1. Deine Domain-Inhaberschaft verifizieren ([ACME-Challenge](../glossar.md#acme-challenge))
2. Das Zertifikat ausstellen
3. Die Nginx-Konfiguration automatisch anpassen

**Zertifikat ohne automatische Konfiguration (nur Dateien, manuell einbinden):**
```bash
sudo certbot certonly --nginx -d example.com
```

**Für Nginx-unabhängige Nutzung (standalone, Port 80 muss frei sein):**
```bash
sudo certbot certonly --standalone -d example.com
```

**Zertifikatsdateien werden gespeichert unter:**
```
/etc/letsencrypt/live/example.com/
├── cert.pem        ← Das Zertifikat selbst
├── chain.pem       ← Zertifikatskette zur CA
├── fullchain.pem   ← cert.pem + chain.pem (das verwendest du meist)
└── privkey.pem     ← Privater Schlüssel (geheim halten!)
```

## 5. Automatische Erneuerung

Let's Encrypt-Zertifikate laufen nach 90 Tagen ab. certbot richtet automatisch einen systemd-Timer ein:

```bash
# Timer-Status anzeigen
sudo systemctl status certbot.timer

# Manuelle Erneuerung testen (dry-run, keine echte Erneuerung)
sudo certbot renew --dry-run

# Alle Zertifikate anzeigen
sudo certbot certificates
```

**Erneuerungslog prüfen:**
```bash
sudo journalctl -u certbot
```

Falls der Timer nicht eingerichtet ist, manuell einrichten:
```bash
# Cron-Job als Alternative
sudo crontab -e
# Folgende Zeile hinzufügen (täglich um 3:00 Uhr):
0 3 * * * certbot renew --quiet
```

## 6. HTTPS in Nginx konfigurieren

Nach `certbot --nginx` sieht eine typische Nginx-Konfiguration so aus:

```nginx
server {
    listen 80;
    server_name example.com www.example.com;
    # HTTP auf HTTPS umleiten
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name example.com www.example.com;

    # Von certbot automatisch eingetragen:
    ssl_certificate /etc/letsencrypt/live/example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/example.com/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

    location / {
        # Hier kommt deine eigentliche Konfiguration
        proxy_pass http://localhost:3000;
    }
}
```

## 7. HTTPS erzwingen (HTTP-Redirect)

Besucher, die `http://example.com` aufrufen, sollen automatisch auf `https://example.com` weitergeleitet werden:

```nginx
server {
    listen 80;
    server_name example.com www.example.com;
    return 301 https://$host$request_uri;
}
```

Der HTTP-Statuscode `301` bedeutet "Dauerhaft verschoben" – Browser und Suchmaschinen merken sich das.

## 8. TLS-Konfiguration testen

**Im Browser:** Auf das Schloss-Symbol klicken und Zertifikatsdetails prüfen.

**Mit curl:**
```bash
# HTTPS-Verbindung testen
curl -I https://example.com

# Zertifikatsdetails anzeigen
curl -vI https://example.com 2>&1 | grep -A5 "SSL"
```

**Mit openssl:**
```bash
# Zertifikat des Servers anzeigen
openssl s_client -connect example.com:443 -servername example.com

# Ablaufdatum des Zertifikats prüfen
echo | openssl s_client -connect example.com:443 2>/dev/null | openssl x509 -noout -dates
```

**Online-Tools:**
- [SSL Labs Test](https://www.ssllabs.com/ssltest/) – bewertet die TLS-Konfiguration mit Note
- [Why No Padlock](https://www.whynopadlock.com/) – findet Mixed-Content-Probleme

## 9. Häufige Probleme

**"Zertifikat abgelaufen":**
```bash
# Prüfen wann es abläuft
sudo certbot certificates

# Manuell erneuern
sudo certbot renew
sudo systemctl reload nginx
```

**"Domain konnte nicht verifiziert werden":**
- Port 80 muss beim Ausstellen erreichbar sein (Firewall prüfen!)
- DNS-Eintrag muss auf diesen Server zeigen
- Kein anderer Prozess darf Port 80 blockieren

```bash
# Port 80 prüfen
sudo ss -tulnp | grep :80
sudo ufw status
```

**Mixed Content (Seite lädt, aber Schloss zeigt Warnung):**
- Irgendwo im HTML/CSS wird noch `http://` statt `https://` verwendet
- Alle internen Links und Ressourcen auf HTTPS umstellen

**Zertifikat gilt nicht für www:**
```bash
# Für Haupt-Domain und www ausstellen
sudo certbot --nginx -d example.com -d www.example.com
```

## 10. Referenztabelle

| Befehl | Was es macht |
|--------|-------------|
| `sudo certbot --nginx -d domain.de` | Zertifikat ausstellen + Nginx konfigurieren |
| `sudo certbot certificates` | Alle Zertifikate anzeigen |
| `sudo certbot renew --dry-run` | Erneuerung simulieren |
| `sudo certbot renew` | Alle fälligen Zertifikate erneuern |
| `sudo systemctl status certbot.timer` | Automatischen Erneuerungs-Timer prüfen |
| `openssl s_client -connect host:443` | TLS-Verbindung debuggen |
| `curl -I https://domain.de` | HTTPS-Antwort prüfen |
