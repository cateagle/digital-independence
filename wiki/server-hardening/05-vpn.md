# VPN mit WireGuard - Spickzettel

## 1. Was ist ein VPN und warum brauchst du es?

Ein [VPN](../glossar.md#vpn-virtual-private-network) (Virtual Private Network) baut einen verschlüsselten Tunnel zwischen deinem Gerät und deinem Server.

Beim [Self-Hosting](../glossar.md#self-hosting) willst du damit interne Dienste absichern: Datenbanken, Admin-UIs oder Monitoring-Tools sollen nicht direkt aus dem Internet erreichbar sein. Du hältst sie hinter der [Firewall](../glossar.md#firewall) und greifst nur über das VPN darauf zu.

**Windows-Analogie:** Das VPN im Home-Office – du verbindest dich mit dem Firmennetz und kannst interne Systeme nutzen, die sonst nicht erreichbar wären.

Wir nutzen hier **WireGuard**, das direkt im Linux-Kernel läuft. Als Alternative gibt es OpenVPN, das aber aufwändiger zu konfigurieren ist.

## 2. WireGuard installieren

Auf Ubuntu reicht ein einziger Befehl:

```bash
sudo apt update && sudo apt install wireguard
```

Prüfe danach, ob die Installation funktioniert hat:

```bash
wg --version
# Ausgabe: wireguard-tools v1.0.x...
```

## 3. Schlüsselpaar erzeugen

WireGuard nutzt Public-Key-Kryptografie – ähnlich wie [SSH](../glossar.md#ssh-secure-shell). Jedes Gerät (Server und Client) bekommt ein eigenes Schlüsselpaar: einen privaten Schlüssel, der geheim bleibt, und einen öffentlichen Schlüssel, den du weitergibst.

### Auf dem Server

```bash
# Ins WireGuard-Verzeichnis wechseln
cd /etc/wireguard

# Schlüsselpaar erzeugen und direkt in Dateien speichern
wg genkey | sudo tee server_private.key | wg pubkey | sudo tee server_public.key

# Berechtigungen einschränken – private Keys darf nur root lesen
sudo chmod 600 server_private.key

# Schlüssel anzeigen (brauchst du gleich für die Konfiguration)
sudo cat server_private.key
sudo cat server_public.key
```

### Auf dem Client (deinem Laptop/PC)

```bash
# Gleiches Vorgehen auf dem Client-Gerät
wg genkey | tee client_private.key | wg pubkey | tee client_public.key

chmod 600 client_private.key

cat client_private.key
cat client_public.key
```

Notiere dir alle vier Schlüssel – du brauchst sie in den nächsten Schritten.

## 4. Server-Konfiguration erstellen

Die Konfigurationsdatei liegt unter `/etc/wireguard/wg0.conf`. Das `wg0` ist der Name des virtuellen Netzwerk-Interfaces – du kannst es so lassen.

```bash
sudo nano /etc/wireguard/wg0.conf
```

Inhalt der Datei (ersetze die Platzhalter durch deine echten Schlüssel):

```ini
[Interface]
# Privater Schlüssel des Servers
PrivateKey = DEIN_SERVER_PRIVATE_KEY

# IP-Adresse des Servers im VPN-Netz (wähle einen privaten Adressbereich)
Address = 10.0.0.1/24

# Port, auf dem WireGuard lauscht
ListenPort = 51820

# Paket-Weiterleitung aktivieren (nötig, damit Clients ins Netz kommen)
PostUp = sysctl -w net.ipv4.ip_forward=1
PostDown = sysctl -w net.ipv4.ip_forward=0

[Peer]
# Öffentlicher Schlüssel des Clients
PublicKey = DEIN_CLIENT_PUBLIC_KEY

# IP-Adresse, die dieser Client im VPN bekommt
AllowedIPs = 10.0.0.2/32
```

Berechtigungen setzen:

```bash
sudo chmod 600 /etc/wireguard/wg0.conf
```

## 5. Firewall-Regeln für WireGuard anpassen

WireGuard kommuniziert über [UDP](../glossar.md#udp-user-datagram-protocol) auf [Port](../glossar.md#port) 51820. Dieser Port muss in der Firewall offen sein:

```bash
# WireGuard-Port in UFW freigeben
sudo ufw allow 51820/udp

# Regel überprüfen
sudo ufw status
```

Ausgabe sollte jetzt unter anderem zeigen:
```
51820/udp                  ALLOW IN    Anywhere
```

> Hinweis: Die internen Dienste, auf die du über das VPN zugreifst, bleiben weiterhin gesperrt. Nur der VPN-Port selbst muss offen sein – das ist der Punkt der ganzen Übung.

## 6. WireGuard starten und beim Boot aktivieren

```bash
# Interface sofort starten
sudo systemctl start wg-quick@wg0

# Beim Boot automatisch starten
sudo systemctl enable wg-quick@wg0

# Beides auf einmal
sudo systemctl enable --now wg-quick@wg0

# Status prüfen
sudo systemctl status wg-quick@wg0
```

## 7. Client-Konfiguration

Auf dem Client (dein Laptop oder Handy) brauchst du ebenfalls eine Konfigurationsdatei. Erstelle sie dort als `wg0.conf` (oder tippe sie in die WireGuard-App ein):

```ini
[Interface]
# Privater Schlüssel des Clients
PrivateKey = DEIN_CLIENT_PRIVATE_KEY

# IP-Adresse des Clients im VPN
Address = 10.0.0.2/24

[Peer]
# Öffentlicher Schlüssel des Servers
PublicKey = DEIN_SERVER_PUBLIC_KEY

# Öffentliche IP-Adresse (oder Domain) des Servers + WireGuard-Port
Endpoint = DEINE_SERVER_IP:51820

# Welche Zieladressen über das VPN geleitet werden
# 10.0.0.0/24 = nur VPN-interner Traffic; 0.0.0.0/0 = gesamter Traffic
AllowedIPs = 10.0.0.0/24

# Hält die Verbindung hinter NAT aufrecht (z. B. bei Heimrouter)
PersistentKeepalive = 25
```

Verbindung auf dem Client herstellen:

```bash
# Unter Linux
sudo wg-quick up wg0

# Verbindung wieder trennen
sudo wg-quick down wg0
```

Auf dem Handy kannst du die Konfiguration als QR-Code einlesen. Generiere ihn auf dem Server:

```bash
sudo apt install qrencode
sudo qrencode -t ansiutf8 < /pfad/zur/client-wg0.conf
```

## 8. Verbindung testen

Nach dem Verbindungsaufbau auf dem Client prüfst du den Status auf dem Server:

```bash
sudo wg show
```

Beispielausgabe bei aktiver Verbindung:

```
interface: wg0
  public key: abc123...
  private key: (hidden)
  listening port: 51820

peer: xyz789...
  endpoint: 1.2.3.4:54321
  allowed ips: 10.0.0.2/32
  latest handshake: 5 seconds ago
  transfer: 1.23 KiB received, 2.34 KiB sent
```

Sobald du `latest handshake` siehst, ist die Verbindung aktiv. Teste die Erreichbarkeit:

```bash
# Vom Client aus den Server im VPN anpingen
ping 10.0.0.1
```

## 9. Nützliche Befehle – Übersicht

| Befehl | Was er macht |
|--------|--------------|
| `sudo systemctl enable --now wg-quick@wg0` | WireGuard starten und autostart aktivieren |
| `sudo systemctl start wg-quick@wg0` | Interface starten |
| `sudo systemctl stop wg-quick@wg0` | Interface stoppen |
| `sudo wg show` | Status und verbundene Peers anzeigen |
| `sudo wg-quick up wg0` | Interface manuell hochfahren |
| `sudo wg-quick down wg0` | Interface manuell runterfahren |
| `wg genkey` | Privaten Schlüssel erzeugen |
| `wg pubkey` | Öffentlichen Schlüssel aus privatem ableiten |
| `sudo ufw allow 51820/udp` | WireGuard-Port in Firewall öffnen |

## 10. Häufige Fehler

**Problem:** `wg show` zeigt keinen Handshake.
**Lösung:** Prüfe, ob der Port 51820/udp in der Firewall des Servers offen ist (`sudo ufw status`). Prüfe außerdem, ob der öffentliche Schlüssel des Clients in der Server-Konfiguration unter `[Peer]` korrekt eingetragen ist – ein einziges falsches Zeichen verhindert die Verbindung.

**Problem:** Ping auf 10.0.0.1 schlägt fehl, obwohl der Handshake klappt.
**Lösung:** IP-Weiterleitung ist nicht aktiv. Prüfe, ob `net.ipv4.ip_forward=1` gesetzt ist:
```bash
sysctl net.ipv4.ip_forward
```
Wenn der Wert `0` ist, fehlt die `PostUp`-Zeile in der Server-Konfiguration oder du musst WireGuard neu starten.

**Problem:** Nach einem Neustart des Servers ist WireGuard nicht aktiv.
**Lösung:** Du hast vergessen, den Autostart zu aktivieren. Nachholen mit:
```bash
sudo systemctl enable wg-quick@wg0
```

**Problem:** Die Client-Konfiguration enthält den falschen öffentlichen Schlüssel des Servers.
**Lösung:** Server- und Client-Schlüssel dürfen nicht verwechselt werden. Der `PublicKey` in `[Peer]` ist immer der Schlüssel des *anderen* Geräts – niemals der eigene.
