# VPS und Virtualisierung

## 1. Was ist Virtualisierung?

Ein physischer Server ist teuer und leistungsstark – aber ein einzelner Dienst nutzt selten die gesamte Hardware aus. Virtualisierung löst das: Eine Software, der **Hypervisor**, teilt die physische Hardware in mehrere voneinander getrennte virtuelle Maschinen (VMs) auf. Jede VM verhält sich wie ein eigener Computer mit eigenem Betriebssystem, eigenem RAM und eigener CPU-Zuteilung.

**Analogie:** Ein Mehrfamilienhaus. Das Gebäude (die physische Hardware) gehört einem Eigentümer. Die einzelnen Wohnungen (die VMs) sind voneinander getrennt – was in einer Wohnung passiert, beeinflusst die anderen nicht. Jeder Mieter hat seinen eigenen Schlüssel und seinen eigenen Bereich.

Bekannte Hypervisoren sind z.B. KVM (häufig bei Linux-Servern), VMware und Microsoft Hyper-V.

## 2. Was ist ein VPS?

Ein **[VPS](../glossar.md#vps-virtual-private-server) (Virtual Private Server)** ist eine gemietete virtuelle Maschine bei einem Anbieter. Du bekommst Root-Zugang zu einer VM, die auf einem physischen Server im Rechenzentrum des Anbieters läuft.

| | **Shared Hosting** | **VPS** | **Dedicated Server** |
|---|---|---|---|
| Hardware | geteilt, keine Kontrolle | geteilt, aber isoliert | exklusiv |
| Betriebssystem | vorgegeben | frei wählbar | frei wählbar |
| Root-Zugang | nein | ja | ja |
| Ressourcen | stark begrenzt | fest zugeteilt | vollständig |
| Preis | niedrig | mittel | hoch |
| Typischer Einsatz | einfache Websites | eigene Dienste, Self-Hosting | hohe Last, Compliance |

**Windows-Analogie:** Shared Hosting ist wie ein Zimmer in einer WG – du teilst Küche und Bad. Ein VPS ist eine eigene Wohnung zur Miete. Ein Dedicated Server ist ein gekauftes Haus.

## 3. Gängige Anbieter

Es gibt viele VPS-Anbieter. Einige der verbreitetsten:

- **Hetzner** (Deutschland)
- **Ionos** (Deutschland)
- **DigitalOcean** (USA, weltweite Standorte)
- **Linode / Akamai** (USA, weltweite Standorte)
- **Contabo** (Deutschland)

## 4. Worauf achten bei der Auswahl?

- **RAM:** Wichtigste Ressource für die meisten Dienste – lieber etwas mehr einplanen
- **CPU:** Anzahl der vCPUs; für einfache Dienste reichen 1–2 Kerne
- **Speicher:** SSD ist Standard; auf den Typ achten (NVMe ist schneller als SATA-SSD)
- **Bandbreite / Traffic:** Manche Anbieter begrenzen das monatliche Datenvolumen
- **Standort / Latenz:** Server in deiner Region bedeuten niedrigere Latenz für deine Nutzer
- **Preis:** Monatliche Kosten; oft gibt es stündliche Abrechnung

## 5. Erster Login – typischer Ablauf

Nach der Bestellung erhältst du eine IP-Adresse und entweder ein Root-Passwort oder kannst einen [SSH-Key](../glossar.md#ssh-key-ssh-schlüsselpaar) hinterlegen.

```bash
# Als root einloggen (IP durch deine Server-IP ersetzen)
ssh root@203.0.113.10
```

Sobald du eingeloggt bist, solltest du als erstes einen normalen Benutzer anlegen und den direkten Root-Login deaktivieren:

```bash
# Neuen Benutzer anlegen
adduser deinname

# Benutzer zur sudo-Gruppe hinzufügen (damit er Admin-Befehle ausführen kann)
usermod -aG sudo deinname
```

```bash
# In einer neuen Terminal-Session testen, ob der Login funktioniert
ssh deinname@203.0.113.10

# Erst danach root-Login in der SSH-Konfiguration deaktivieren
```

Den Root-Login zu deaktivieren ist ein wichtiger Sicherheitsschritt. Eine genaue Anleitung dazu findest du unter [`../server-hardening/03-root-login-verhindern.md`](../server-hardening/03-root-login-verhindern.md).

## 6. Alternativen zum VPS

| Option | Kurzbeschreibung |
|---|---|
| **Heimserver / Raspberry Pi** | Volle Kontrolle, einmalige Hardwarekosten – aber abhängig von Heimanbindung, Stromkosten und dynamischer IP |
| **Dedicated Server** | Exklusive Hardware, maximale Leistung – aber deutlich teurer und oft mit längeren Vertragslaufzeiten |
| **Cloud (AWS, GCP, Azure)** | Sehr flexibel und skalierbar – aber komplex in der Einrichtung und bei kleinen Projekten oft teurer als ein VPS |
