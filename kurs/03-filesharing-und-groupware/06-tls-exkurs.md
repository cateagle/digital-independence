# Exkurs: TLS, SSL, Let's Encrypt und Certbot

## Warum dein Browser bei HTTP meckert

Wenn du Nextcloud in der VM über `http://` aufrufst, zeigt der Browser ein Schloss-Symbol mit einem Warnzeichen – oder verweigert den Zugang ganz. Das ist kein Bug, das ist Absicht.

HTTP sendet alles im Klartext. Jeder im gleichen Netzwerk (Router, ISP, VPN-Anbieter) kann mitlesen:
- Login-Daten
- Session-Tokens (mit denen man sich als du einloggen kann)
- Dateiinhalte

Für einen Cloud-Dienst, der Dokumente und Fotos speichert, ist das inakzeptabel.

---

## SSL, TLS – was ist was?

Diese Begriffe werden oft verwechselt:

- **[SSL](../../wiki/glossar.md#ssl-secure-sockets-layer)** (Secure Sockets Layer): Das Original-Protokoll aus den 90ern. Gilt als unsicher. SSL 2.0 und 3.0 sind beide offiziell depreceated.
- **[TLS](../../wiki/glossar.md#tls-transport-layer-security)** (Transport Layer Security): Der Nachfolger. Aktuell ist TLS 1.3.

Wenn jemand "SSL-Zertifikat" sagt, meint er fast immer ein **TLS-Zertifikat**. Der Begriff SSL hat sich im Sprachgebrauch gehalten, obwohl SSL selbst nicht mehr genutzt wird.

**Was macht TLS?**
1. **Verschlüsselung:** Die Verbindung zwischen Browser und Server ist verschlüsselt.
2. **Authentifizierung:** Der Browser kann prüfen, ob er wirklich mit `cloud.meinedomain.de` spricht und nicht mit einem Angreifer ([Man-in-the-Middle](../../wiki/glossar.md#man-in-the-middle-angriff-mitm)).

---

## Wie Zertifikate funktionieren

Ein TLS-Zertifikat ist eine **digitale Unterschrift** einer Zertifizierungsstelle ([CA](../../wiki/glossar.md#ca-certificate-authority) - Certificate Authority).

Vereinfacht:
1. Du besitzt cloud.meinedomain.de (kannst das beweisen, weil du DNS kontrollierst)
2. Eine CA (z.B. [Let's Encrypt](../../wiki/glossar.md#lets-encrypt)) prüft das
3. Die CA stellt ein Zertifikat aus: "cloud.meinedomain.de gehört dieser Person"
4. Browser vertrauen der CA (sie ist im Browser vorinstalliert)
5. Ergo: Browser vertraut deinem Server


Das Zertifikat enthält einen **Public Key**. Dein Server hat den dazugehörigen **Private Key**. Nur mit dem Private Key kann der verschlüsselte Traffic entschlüsselt werden.

**Wichtig:** Der Private Key darf nie das System verlassen. Wenn er kompromittiert wird, muss das Zertifikat sofort zurückgezogen (revoked) werden.

---

## Selbst signierte Zertifikate

Du kannst auch ein Zertifikat selbst ausstellen (`openssl self-signed`). Browser vertrauen dem aber nicht, weil du keine bekannte CA bist. Du bekommst dann genau die Warnung, die wir in der VM sehen.

Selbst signierte Zertifikate sind für interne Systeme (nur du, nur lokal) akzeptabel. Für öffentlich erreichbare Dienste sind sie keine Option.

---

## Let's Encrypt: Kostenlose, automatische Zertifikate

[Let's Encrypt](https://letsencrypt.org/) ist eine **Non-Profit Certificate Authority**, die seit 2015 kostenlose TLS-Zertifikate ausstellt. Dahinter stehen Internet Security Research Group ([ISRG](../../wiki/glossar.md#isrg-internet-security-research-group)), Mozilla, [EFF](../../wiki/glossar.md#eff-electronic-frontier-foundation) und andere.

**Warum existiert das?**
Vor Let's Encrypt kosteten Zertifikate Geld (oft 50–200 €/Jahr). Das war eine technische Zugangshürde. Let's Encrypt hat [HTTPS](../../wiki/glossar.md#https-hypertext-transfer-protocol-secure) demokratisiert: Heute ist kein finanzieller Grund mehr, HTTP zu betreiben.

**Einschränkungen:**
- Nur **[Domain-Validation (DV)](../../wiki/glossar.md#dv-zertifikat-domain-validation)** – Let's Encrypt prüft, ob du die Domain kontrollierst, nicht wer du bist
- Zertifikate laufen nach **90 Tagen** ab (Absicht: zwingt zur Automatisierung)
- [Wildcard-Zertifikate](../../wiki/glossar.md#wildcard-zertifikat) möglich, aber komplexer

---

## Certbot: Das Tool für Let's Encrypt

[Certbot](https://certbot.eff.org/) ist das Standard-Tool, um Let's Encrypt-Zertifikate zu beantragen und automatisch zu erneuern. Entwickelt von der EFF.

### Wie Certbot funktioniert (vereinfacht)

```bash
# Certbot fragt: "Hast du Kontrolle über cloud.meinedomain.de?"
# Methode 1: HTTP-Challenge
# Let's Encrypt ruft http://cloud.meinedomain.de/.well-known/acme-challenge/<token> ab
# Wenn Certbot die Antwort dort ablegen kann: Beweis erbracht

# Methode 2: DNS-Challenge
# Certbot legt einen TXT-Record in deinem DNS ab
# Let's Encrypt prüft diesen Record
# Beweis ohne laufenden Webserver möglich
```

### Certbot in der Praxis

```bash
# Installation
sudo apt install certbot python3-certbot-nginx

# Zertifikat beantragen (mit Nginx als Webserver)
sudo certbot --nginx -d cloud.meinedomain.de

# Automatische Erneuerung (Certbot richtet einen Cron-Job ein)
sudo certbot renew --dry-run
```

Certbot legt die Zertifikatsdateien ab unter:
- `/etc/letsencrypt/live/cloud.meinedomain.de/fullchain.pem` – Zertifikat
- `/etc/letsencrypt/live/cloud.meinedomain.de/privkey.pem` – Privater Schlüssel

> **Sicherheit:** `privkey.pem` hat Berechtigungen 600 (nur root lesbar).
> Wer diesen Key hat, kann sich als dein Server ausgeben.

---

## Was AIO automatisch macht

Nextcloud AIO übernimmt TLS vollautomatisch – das ist einer der Hauptgründe, warum wir AIO verwenden:

1. Du trägst deine Domain ein (z.B. `cloud.meinedomain.de`)
2. AIO startet seinen eigenen Nginx-Proxy
3. AIO beantragt automatisch ein Let's Encrypt-Zertifikat über Certbot
4. AIO erneuert das Zertifikat automatisch vor dem Ablauf

**Was dafür nötig ist:**
- Die Domain muss wirklich auf deinen Server zeigen (DNS A-Record)
- Port 80 und 443 müssen aus dem Internet erreichbar sein
- Die VM in der VM-Welt erfüllt das nicht → deshalb sehen wir die Browser-Warnung

---

## Zusammenfassung: Was du wissen musst

| Begriff       | Was es ist                                              |
|---------------|---------------------------------------------------------|
| SSL           | Veraltetes Protokoll, heute meistens als Synonym für TLS verwendet |
| TLS           | Das aktuelle Protokoll für verschlüsselte Verbindungen  |
| Zertifikat    | Digitale Unterschrift einer CA, beweist Identität       |
| CA            | Certificate Authority – stellt Zertifikate aus          |
| Let's Encrypt | Kostenlose CA, de facto Standard                       |
| Certbot       | Tool zum Beantragen und Erneuern von Let's Encrypt-Zertifikaten |
| HTTPS         | HTTP über TLS                                           |

Für den Produktionsbetrieb: Immer HTTPS, immer automatische Erneuerung, niemals selbst signierte Zertifikate für öffentliche Dienste.
