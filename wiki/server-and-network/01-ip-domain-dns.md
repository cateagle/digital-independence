# IP-Adressen, Domains und DNS - Spickzettel

## 1. Was ist eine IP-Adresse?

Eine IP-Adresse ist die "Hausnummer" eines Computers im Netzwerk. Jedes Gerät im Internet oder im lokalen Netzwerk hat eine IP-Adresse.

**Windows-Analogie:** Wie eine Telefonnummer – jedes Gerät ist erreichbar, wenn man die richtige Nummer kennt.

Es gibt zwei Versionen:
- **[IPv4](../glossar.md#ipv4-internet-protocol-version-4):** vier Zahlen von 0–255, z.B. `192.168.1.10` oder `93.184.216.34`
- **[IPv6](../glossar.md#ipv6-internet-protocol-version-6):** längere Schreibweise, z.B. `2001:0db8:85a3::8a2e:0370:7334` (noch selten im Alltag)

## 2. Öffentliche vs. private IP-Adressen

| Typ | Beispiele | Erreichbar von? |
|-----|-----------|-----------------|
| Privat | `192.168.x.x`, `10.x.x.x`, `172.16.x.x` | Nur im lokalen Netzwerk |
| Öffentlich | alles andere, z.B. `93.184.216.34` | Aus dem Internet |

**Merkhilfe:**
- Dein Router hat eine öffentliche IP (zugeteilt vom Internetanbieter).
- Deine Geräte zu Hause haben private IPs – sie sind nicht direkt aus dem Internet erreichbar.
- Ein Server im Rechenzentrum hat eine öffentliche IP, damit er erreichbar ist.

**Eigene IP-Adresse herausfinden:**
```bash
# Private IP (im lokalen Netzwerk)
ip a

# Öffentliche IP (so sieht dich das Internet)
curl ifconfig.me
```

## 3. Was ist eine Domain?

Domains sind lesbare Namen für IP-Adressen. Niemand merkt sich `93.184.216.34` – deshalb gibt es `example.com`.

**Windows-Analogie:** Wie das Telefonbuch – du suchst "Pizza Mayer" und bekommst die Nummer. Du rufst die Nummer an, nicht den Namen.

Struktur einer Domain:
```
blog.example.com
│    │       └── Top-Level-Domain (TLD): .com, .de, .org, .net
│    └────────── Second-Level-Domain: example
└─────────────── Subdomain: blog
```

## 4. Was ist DNS?

[DNS](../glossar.md#dns-domain-name-system) (Domain Name System) ist das Telefonbuch des Internets. Es übersetzt Domains in IP-Adressen.

**Ablauf einer DNS-Auflösung:**
```
Du tippst: example.com
    ↓
Dein Computer fragt den DNS-Resolver (oft deinen Router oder 8.8.8.8)
    ↓
Der Resolver fragt die autoritativen Nameserver für example.com
    ↓
Antwort: 93.184.216.34
    ↓
Dein Browser verbindet sich mit dieser IP
```

## 5. Wichtige DNS-Eintragstypen

| Typ | Bedeutung | Beispiel |
|-----|-----------|---------|
| A | Domain → IPv4-Adresse | `example.com → 93.184.216.34` |
| AAAA | Domain → IPv6-Adresse | `example.com → 2606:2800:...` |
| [CNAME](../glossar.md#cname-canonical-name) | Domain → andere Domain (Alias) | `www.example.com → example.com` |
| [MX](../glossar.md#mx-record-mail-exchange-record) | Mail-Server für die Domain | `example.com → mail.example.com` |
| TXT | Freitext (oft für Verifikationen) | SPF, DKIM, Let's Encrypt |

**A-Record** – der häufigste Eintrag. Zeigt deine Domain auf deinen Server:
```
Typ:   A
Name:  example.com
Wert:  93.184.216.34
```

**CNAME** – ein Alias. Statt eine IP einzutragen, zeigst du auf eine andere Domain:
```
Typ:   CNAME
Name:  www.example.com
Wert:  example.com
```

Vorteil: Änderst du die IP beim A-Record, folgt der CNAME automatisch.

## 6. Was ist TTL?

[TTL](../glossar.md#ttl-time-to-live) (Time to Live) bestimmt, wie lange DNS-Einträge im Cache gespeichert werden.

- TTL `3600` = 1 Stunde: DNS-Server und Browser erinnern sich 1 Stunde an den Eintrag.
- Niedrige TTL (z.B. `300` = 5 Minuten): Änderungen werden schnell wirksam – gut vor einem Serverwechsel.
- Hohe TTL (z.B. `86400` = 24 Stunden): Weniger DNS-Anfragen, aber Änderungen brauchen länger.

**Tipp:** Vor einem geplanten Serverwechsel die TTL auf 300 setzen, warten, umziehen, dann wieder erhöhen.

## 7. DNS-Einträge prüfen mit nslookup und dig

### nslookup (einfacher, auch unter Windows verfügbar)
```bash
# A-Record einer Domain abfragen
nslookup example.com

# Bestimmten DNS-Server verwenden
nslookup example.com 8.8.8.8

# MX-Einträge abfragen
nslookup -type=MX example.com
```

### dig (mächtiger, Linux/Mac)
```bash
# A-Record abfragen
dig example.com

# Nur die IP-Adresse anzeigen
dig +short example.com

# Bestimmten Eintragstyp abfragen
dig example.com MX
dig example.com TXT

# Bestimmten DNS-Server verwenden
dig @8.8.8.8 example.com

# TTL anzeigen
dig +ttl example.com
```

**Beispielausgabe von `dig +short example.com`:**
```
93.184.216.34
```

## 8. Bekannte öffentliche DNS-Server

| Anbieter | IP-Adresse |
|---------|------------|
| Google | `8.8.8.8` und `8.8.4.4` |
| Cloudflare | `1.1.1.1` und `1.0.0.1` |
| Quad9 (Datenschutz) | `9.9.9.9` |

## 9. Lokale Hosts-Datei

Vor DNS schaut dein System in `/etc/hosts` – dort kannst du lokale Einträge eintragen, die DNS überschreiben:

```bash
sudo nano /etc/hosts
```

```
# Format: IP-Adresse    Hostname
127.0.0.1       localhost
192.168.1.50    meinserver.lokal
```

**Windows-Analogie:** Unter Windows gibt es `C:\Windows\System32\drivers\etc\hosts` – dasselbe Konzept.

## 10. Troubleshooting

**Problem: Domain löst nicht auf**
```bash
# DNS-Auflösung testen
dig +short meinedomain.de

# Direkt beim autoritativen Nameserver fragen
dig +short meinedomain.de @ns1.meinprovider.de

# Lokalen DNS-Cache leeren (Ubuntu)
sudo systemd-resolve --flush-caches
```

**Problem: Alte IP kommt noch zurück (TTL-Cache)**
```bash
# Prüfen, welche TTL noch läuft
dig meinedomain.de | grep TTL
```

## 11. Referenztabelle

| Befehl | Was es macht |
|--------|-------------|
| `dig example.com` | DNS-Abfrage für eine Domain |
| `dig +short example.com` | Nur IP-Adresse ausgeben |
| `dig example.com MX` | MX-Einträge abfragen |
| `nslookup example.com` | DNS-Abfrage (Windows-kompatibel) |
| `curl ifconfig.me` | Eigene öffentliche IP anzeigen |
| `ip a` | Netzwerkinterfaces und IPs anzeigen |
| `cat /etc/hosts` | Lokale Hosts-Datei anzeigen |
