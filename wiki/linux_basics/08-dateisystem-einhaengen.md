# 08 - Dateisystem & Einhängen

Unter Windows bekommst du beim Anschließen einer Festplatte automatisch einen neuen Laufwerksbuchstaben wie `D:` oder `E:`. Unter Linux gibt es keine Laufwerksbuchstaben. Stattdessen werden Laufwerke und Partitionen in den bestehenden Verzeichnisbaum *eingehängt* ([gemountet](../glossar.md#mount)). Dieses Kapitel erklärt, wie das funktioniert und wie du Speicherplatz im Blick behältst.

---

## Was bedeutet "Einhängen" (Mounting)?

Stell dir vor, dein gesamtes Dateisystem ist ein einziger großer Baum, der bei `/` beginnt. Ein Laufwerk einhängen bedeutet: Du sagst Linux, dass der Inhalt eines Laufwerks ab einem bestimmten Ordner sichtbar sein soll. Dieser Ordner heißt *Einhängepunkt* (Mount-Point).

**Windows-Analogie:** Unter Windows bekommt eine externe Festplatte den Buchstaben `D:`. Unter Linux sagst du stattdessen: "Zeige den Inhalt dieser Festplatte unter `/mnt/daten` an."

```
/                      ← Wurzel (root)
├── home/              ← auf der Systemfestplatte
├── etc/               ← auf der Systemfestplatte
└── mnt/
    └── daten/         ← hier ist die zweite Festplatte eingehängt
```

---

## Laufwerke anzeigen: lsblk und fdisk

Bevor du ein Laufwerk einhängst, willst du wissen, welche Laufwerke überhaupt vorhanden sind.

```bash
lsblk
```

Beispielausgabe:
```
NAME   MAJ:MIN RM   SIZE RO TYPE MOUNTPOINT
sda      8:0    0   500G  0 disk
├─sda1   8:1    0   499G  0 part /
└─sda2   8:2    0     1G  0 part [SWAP]
sdb      8:16   0   200G  0 disk
└─sdb1   8:17   0   200G  0 part
```

Hier ist `sda` die Hauptfestplatte (mit Betriebssystem) und `sdb` ein zweites Laufwerk, das noch nicht eingehängt ist.

Mehr Details (inkl. Partitionstyp und UUID):
```bash
sudo fdisk -l
```

---

## UUID vs. Gerätename

Jede Partition hat zwei Bezeichnungen:

| Bezeichnung | Beispiel | Beschreibung |
|-------------|---------|--------------|
| Gerätename  | `/dev/sdb1` | Systemname des Geräts - kann sich ändern! |
| UUID        | `a1b2c3d4-...` | Eindeutige ID - bleibt immer gleich |

**Warum ist das wichtig?** Wenn du eine zweite Festplatte anschließt, könnte sie heute `/dev/sdb1` und nach einem Neustart `/dev/sdc1` heißen - je nachdem, in welcher Reihenfolge Linux die Laufwerke erkennt. Die UUID dagegen ist dauerhaft und eindeutig. Deshalb sollte in der `/etc/fstab` immer die UUID verwendet werden.

UUID einer Partition anzeigen:
```bash
sudo blkid /dev/sdb1
```

Ausgabe:
```
/dev/sdb1: UUID="a1b2c3d4-e5f6-7890-abcd-ef1234567890" TYPE="ext4"
```

---

## Laufwerke manuell einhängen: mount

Mit `mount` hängst du ein Laufwerk manuell ein. Das gilt nur bis zum nächsten Neustart.

**Vorbereitung:** Einhängepunkt-Ordner erstellen:
```bash
sudo mkdir -p /mnt/daten
```

**Einhängen:**
```bash
sudo mount /dev/sdb1 /mnt/daten
```

Jetzt ist der Inhalt von `/dev/sdb1` unter `/mnt/daten` zugänglich:
```bash
ls /mnt/daten
```

**Aushängen (unmounten):**
```bash
sudo umount /mnt/daten
```

Achtung: `umount`, nicht `unmount`. Der Befehl wird häufig falsch geschrieben.

**Tipp:** Bevor du ein Laufwerk aushängst, stelle sicher, dass kein Programm darauf zugreift. Sonst bekommst du die Fehlermeldung `target is busy`.

---

## Was ist /etc/fstab?

Die Datei `/etc/fstab` (File System Table) ist die Konfigurationsdatei, die bestimmt, welche Laufwerke beim Systemstart automatisch eingehängt werden.

**Windows-Analogie:** Stell dir vor, du könntest Windows sagen: "Hänge beim Start immer diese externe Festplatte als `D:` ein." Genau das macht `/etc/fstab` unter Linux.

Datei anzeigen:
```bash
cat /etc/fstab
```

Typischer Inhalt:
```
# <Dateisystem>                            <Punkt>   <Typ>  <Optionen>      <dump> <pass>
UUID=a1b2c3d4-e5f6-7890-abcd-ef1234567890 /mnt/daten ext4   defaults        0      2
UUID=b2c3d4e5-f6a7-8901-bcde-f12345678901 none       swap   sw              0      0
```

### Bedeutung der Spalten

| Spalte | Beispiel | Bedeutung |
|--------|---------|-----------|
| Dateisystem | `UUID=a1b2...` | Welches Laufwerk (besser UUID statt `/dev/sdb1`) |
| Einhängepunkt | `/mnt/daten` | Wo soll es eingehängt werden |
| Typ | `ext4` | Dateisystemtyp (ext4, ntfs, vfat, ...) |
| Optionen | `defaults` | Mount-Optionen (meistens `defaults`) |
| dump | `0` | Backup-Tool (fast immer 0) |
| pass | `2` | Reihenfolge der Dateisystemprüfung beim Start (0=keine, 1=root, 2=andere) |

### Eintrag in /etc/fstab hinzufügen

1. UUID ermitteln:
```bash
sudo blkid /dev/sdb1
```

2. Einhängepunkt erstellen:
```bash
sudo mkdir -p /mnt/daten
```

3. fstab bearbeiten:
```bash
sudo nano /etc/fstab
```

4. Zeile am Ende hinzufügen:
```
UUID=a1b2c3d4-e5f6-7890-abcd-ef1234567890 /mnt/daten ext4 defaults 0 2
```

5. Testen, ob der Eintrag korrekt ist (ohne Neustart):
```bash
sudo mount -a
```

Kommt keine Fehlermeldung, ist alles in Ordnung. Beim nächsten Neustart wird das Laufwerk automatisch eingehängt.

**Wichtig:** Ein Fehler in `/etc/fstab` kann dazu führen, dass das System nicht mehr startet. Immer mit `sudo mount -a` testen, bevor du neu startest.

---

## Speicherplatz anzeigen: df -h

`df` (Disk Free) zeigt dir, wie viel Speicherplatz auf allen eingehängten Laufwerken frei ist.

```bash
df -h
```

Die Option `-h` steht für "human-readable" - zeigt Werte in MB, GB statt Bytes.

Beispielausgabe:
```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       499G  120G  354G  26% /
/dev/sdb1       200G   80G  120G  40% /mnt/daten
tmpfs           3.9G     0  3.9G   0% /dev/shm
```

Was du siehst:
- `Size`: Gesamtgröße des Laufwerks
- `Used`: Verwendeter Speicher
- `Avail`: Freier Speicher
- `Use%`: Auslastung in Prozent
- `Mounted on`: Einhängepunkt

**Windows-Analogie:** Entspricht dem Rechtsklick auf ein Laufwerk in Windows Explorer und "Eigenschaften" auswählen - aber für alle Laufwerke auf einmal.

Nur ein bestimmtes Laufwerk anzeigen:
```bash
df -h /mnt/daten
```

---

## Ordnergrösse anzeigen: du -sh

`du` (Disk Usage) zeigt, wie viel Speicherplatz ein Ordner belegt.

```bash
du -sh /var/log
```

Optionen:
- `-s`: Summary - nur den Gesamtwert, nicht jeden Unterordner
- `-h`: Human-readable - in MB/GB statt Bytes

Beispielausgabe:
```
2.3G    /var/log
```

Alle Unterordner eines Verzeichnisses anzeigen, sortiert nach Größe:
```bash
du -sh /var/* | sort -h
```

Ausgabe:
```
4.0K    /var/backups
12M     /var/cache
2.3G    /var/log
```

**Windows-Analogie:** Entspricht dem Rechtsklick auf einen Ordner und "Eigenschaften" anzeigen - zeigt die Gesamtgröße des Ordners.

**Praxistipp:** Wenn deine Festplatte voll wird, hilft dieser Befehl, den größten Speicherfresser zu finden:
```bash
sudo du -sh /var/* /home/* /opt/* 2>/dev/null | sort -h
```

---

## Das Swap-Konzept

### Was ist Swap?

Swap ist ein Bereich auf der Festplatte, den Linux als "Erweiterung" des Arbeitsspeichers (RAM) nutzt, wenn der RAM voll wird.

**Windows-Analogie:** Unter Windows heißt das "Auslagerungsdatei" oder "Pagefile" (standardmäßig `C:\pagefile.sys`). Das Konzept ist identisch: Daten, die nicht mehr in den RAM passen, werden auf die Festplatte ausgelagert.

**Nachteil:** Festplatten sind deutlich langsamer als RAM. Wenn das System stark auf Swap zugreift, wird es spürbar langsamer ("swapping").

### Swap-Datei vs. Swap-Partition

Es gibt zwei Arten:

| Art | Beschreibung | Wann nutzen |
|-----|-------------|-------------|
| Swap-Partition | Eigene Partition auf der Festplatte | Klassisch, bei Installation einrichten |
| Swap-Datei | Normale Datei im Dateisystem | Flexibler, kann später geändert werden |

Ubuntu 24.04 LTS verwendet standardmäßig eine Swap-Datei unter `/swapfile`.

### Swap-Status anzeigen

```bash
swapon --show
```

Ausgabe:
```
NAME      TYPE SIZE   USED PRIO
/swapfile file   2G 512M   -2
```

Oder mit `free`:
```bash
free -h
```

Ausgabe:
```
              total        used        free      shared  buff/cache   available
Mem:           15Gi        4.2Gi       8.1Gi       320Mi       2.9Gi      10Gi
Swap:          2.0Gi       512Mi       1.5Gi
```

### Swap-Datei erstellen oder vergrößern

Falls du mehr Swap benötigst (z.B. auf einem Server mit wenig RAM):

```bash
# 4 GB Swap-Datei erstellen
sudo fallocate -l 4G /swapfile

# Nur für root lesbar machen (Sicherheit!)
sudo chmod 600 /swapfile

# Als Swap formatieren
sudo mkswap /swapfile

# Swap aktivieren
sudo swapon /swapfile

# Prüfen
swapon --show
```

Damit die Swap-Datei nach einem Neustart aktiv bleibt, muss sie in `/etc/fstab` eingetragen sein:
```bash
cat /etc/fstab | grep swap
```

Eintrag (sollte bereits vorhanden sein):
```
/swapfile none swap sw 0 0
```

---

## Zusammenfassung: Wichtige Befehle

| Befehl | Was er macht |
|--------|-------------|
| `lsblk` | Alle Laufwerke und Partitionen anzeigen |
| `sudo blkid /dev/sdb1` | UUID einer Partition anzeigen |
| `sudo mount /dev/sdb1 /mnt/daten` | Laufwerk manuell einhängen |
| `sudo umount /mnt/daten` | Laufwerk aushängen |
| `sudo mount -a` | Alle fstab-Einträge einhängen (zum Testen) |
| `df -h` | Freien Speicherplatz aller Laufwerke anzeigen |
| `du -sh /pfad` | Größe eines Ordners anzeigen |
| `swapon --show` | Swap-Status anzeigen |
| `free -h` | RAM und Swap-Auslastung anzeigen |

---

## Fehlersuche

**"Permission denied" beim Einhängen:**
```bash
sudo mount /dev/sdb1 /mnt/daten
```
Vergiss `sudo` nicht - Laufwerke einhängen erfordert Root-Rechte.

**"target is busy" beim Aushängen:**
Irgendein Prozess greift noch auf das Laufwerk zu. Herausfinden welcher:
```bash
lsof /mnt/daten
```
Dann in einen anderen Ordner wechseln und erneut versuchen.

**System startet nach fstab-Änderung nicht mehr:**
Beim Start Recovery-Modus wählen und den fehlerhaften Eintrag in `/etc/fstab` korrigieren. Deshalb: Immer vorher mit `sudo mount -a` testen!

**Laufwerk wird nach Neustart nicht eingehängt:**
Prüfen, ob ein Eintrag in `/etc/fstab` vorhanden ist:
```bash
cat /etc/fstab
```
