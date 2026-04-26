# 02 - Die Linux Shell (Bash-Grundlagen)

## Fehlersuche FAQ

**F: Warum sagt mein Befehl „Zugriff verweigert"?**
	- Möglicherweise müssen Sie `sudo` verwenden oder Dateiberechtigungen mit `ls -l` überprüfen.

**F: Warum wird mein Skript nicht ausgeführt?**
	- Stellen Sie sicher, dass es Ausführungsberechtigungen hat: `chmod +x script.sh`.

**F: Warum kann ich eine Datei nicht finden?**
	- Überprüfen Sie Ihr aktuelles Verzeichnis mit `pwd` und verwenden Sie `ls` oder `find`.

**F: Wie bekomme ich Hilfe für einen Befehl?**
	- Verwenden Sie `man <befehl>` oder `<befehl> --help`.

**F: Warum sieht mein Terminal anders aus?**
	- Verschiedene Linux-Distributionen oder Shells haben möglicherweise unterschiedliche Eingabeaufforderungen und Farben, aber Befehle funktionieren größtenteils gleich.

---

## Häufige Fehler

- Vergessen, Leerzeichen zwischen Befehlen und Argumenten zu setzen.
- Verwendung von Windows-Backslashes (`\`) statt Linux-Slashes (`/`).
- Nichtverwendung von `sudo` für Admin-Aufgaben.
- Versehentliches Ausführen von Befehlen als Root, die als normaler Benutzer ausgeführt werden sollten.
- Vergessen, Ihr aktuelles Verzeichnis zu überprüfen, bevor Sie Dateibefehle ausführen.
- Fehlende Lektüre von Fehlermeldungen.

---

## Shell-Scripting Miniprojekt: Automatisierte Sicherung

Erstellen Sie ein Skript, das Ihren Dokumentenordner in einem komprimierten Archiv sichert.

1. Öffnen Sie Ihr Terminal und erstellen Sie eine neue Datei:
	 ```
	 nano backup.sh
	 ```
2. Fügen Sie diesen Code ein:
	 ```bash
	 #!/bin/bash
	 BACKUP_NAME="backup-$(date +%Y%m%d-%H%M%S).tar.gz"
	 tar -czvf "$BACKUP_NAME" ~/Documents
	 echo "Sicherung gespeichert als $BACKUP_NAME"
	 ```
3. Speichern und beenden Sie (Strg+O, Enter, Strg+X).
4. Machen Sie es ausführbar:
	 ```
	 chmod +x backup.sh
	 ```
5. Führen Sie es aus:
	 ```
	 ./backup.sh
	 ```

Dieses Skript erstellt eine zeitgestempelte Sicherung Ihres Dokumentenordners. Versuchen Sie, es zu ändern, um andere Ordner zu sichern oder es mit cron zu automatisieren!
Die [Shell](../glossar.md#shell) ist Ihre Hauptschnittstelle zu Linux. Wenn Sie Windows gewohnt sind, stellen Sie sie sich als übergeladene Eingabeaufforderung vor. [Bash](../glossar.md#bash) ist die häufigste Shell unter Linux.

## Was ist die Shell?
- Die Shell ermöglicht es Ihnen, Befehle einzugeben, um mit dem System zu interagieren.
- Sie können Programme ausführen, Dateien verwalten und Aufgaben automatisieren.

## Grundbefehle
- `pwd`: Print Working Directory (Arbeitsverzeichnis anzeigen). Zeigt an, wo Sie sich im Dateisystem befinden.
- `cd`: Change Directory (Verzeichnis wechseln). Wechseln Sie zwischen Ordnern.
- `ls`: Dateien und Ordner auflisten.
- `tree`: Verzeichnisstruktur als Baum anzeigen.
- `cat`: Dateiinhalte anzeigen.
- `less`, `head`, `tail`: Teile von Dateien anzeigen.
- `grep`: Nach Text in Dateien suchen.
- `nano`: Einfacher Text-Editor (großartig für Anfänger).

## Produktivitätsfunktionen

- Schneller und flexibler als GUIs für viele Aufgaben.
- Essentiell für Remote-Server (kein Desktop).
- Ermöglicht Automatisierung und Scripting.

---

**Zusammenfassung:**
---

## Trainingsübungen

Versuchen Sie diese in Ihrem Terminal:

1. Navigieren Sie zu `/etc` und listen Sie Dateien auf.
2. Erstellen Sie ein neues Verzeichnis namens `test` in Ihrem Home-Ordner.
3. Erstellen Sie eine Datei namens `hello.txt` und schreiben Sie „Hello, Linux!" hinein.
4. Suchen Sie nach dem Wort „Linux" in `hello.txt`.
5. Schreiben Sie ein Skript, das Ihren Benutzernamen ausgibt.
6. Installieren Sie `htop` und führen Sie es aus.
7. Legen Sie einen Alias für `ls -l` namens `ll` fest.
8. Suchen Sie alle `.txt` Dateien in Ihrem Home-Verzeichnis.
9. Bearbeiten Sie eine Datei mit `nano`.
10. Überprüfen Sie das Handbuch für `grep`.

---

## Weitere Ressourcen

- [LinuxCommand.org](http://linuxcommand.org/)
- [Ubuntu Dokumentation](https://help.ubuntu.com/)
- [Bash Guide for Beginners](https://tldp.org/LDP/Bash-Beginners-Guide/html/)
- [Explainshell](https://explainshell.com/)

---

## Anhang: Weitere Befehle und Konzepte

### Dateiverwaltung

- `mkdir` — Verzeichnis erstellen
	- Beispiel:
		```
		$ mkdir neuerordner
		```
- `rmdir` — Leeres Verzeichnis entfernen
- `rm` — Dateien entfernen
	- Beispiel:
		```
		$ rm datei.txt
		```
- `cp` — Dateien kopieren
	- Beispiel:
		```
		$ cp datei.txt sicherung.txt
		```
- `mv` — Dateien verschieben oder umbenennen
	- Beispiel:
		```
		$ mv datei.txt neuedatei.txt
		```

### Berechtigungen

- `chmod` — Dateiberechtigungen ändern
	- Beispiel:
		```
		$ chmod 755 script.sh
		```
- `chown` — Dateieigentümer ändern
	- Beispiel:
		```
		$ sudo chown benutzer:benutzer datei.txt
		```

### Festplattenspeicher

- `df -h` — Festplattenspeicher anzeigen
- `du -sh` — Ordnergröße anzeigen

### Netzwerk

- `ping` — Netzwerkverbindung testen
	- Beispiel:
		```
		$ ping google.com
		```
- `ss -tulnp` — Offene Ports anzeigen
- `curl` — Dateien aus dem Web herunterladen
	- Beispiel:
		```
		$ curl http://example.com
		```

### Systeminformationen

- `uname -a` — Systeminformationen anzeigen
- `whoami` — Ihren Benutzernamen anzeigen
- `date` — Aktuelles Datum/Uhrzeit anzeigen
- `uptime` — Anzeigen, wie lange das System läuft

### Umgebungsvariablen

- `echo $PATH` — PATH-Variable anzeigen
- `export VAR=wert` — Variable setzen

### Erweiterte Shell-Funktionen

- **Globbing:** Verwenden Sie `*` und `?` zum Abgleichen von Dateien
	- Beispiel:
		```
		$ ls *.txt
		```
- **Befehlsersetzung:**
	- Beispiel:
		```
		$ echo "Heute ist $(date)"
		```
- **Hintergrund-Jobs:**
	- Beispiel:
		```
		$ sleep 60 &
		```
- **Job-Steuerung:**
	- `jobs`, `fg`, `bg`

### Shell-Anpassung

- Bearbeiten Sie `.bashrc`, um Aliase, Funktionen und Umgebungsvariablen hinzuzufügen.

---

## Abschließende Worte

Das Erlernen der Shell braucht Zeit und Übung. Haben Sie keine Angst zu experimentieren. Verwenden Sie die Handbuchseiten, versuchen Sie Befehle und bauen Sie Ihr Vertrauen auf. Die Shell ist Ihr Tor zur vollen Kraft von Linux!
Die Shell ist das Herz von Linux. Das Erlernen grundlegender Befehle und Funktionen macht Sie viel produktiver und selbstbewusster bei der Verwendung von Linux, ob auf Ihrem eigenen Computer oder einem Remote-Server.


# Shell-Anleitung für Windows-Benutzer: Erweiterte Abschnitte

---

## Terminal öffnen

Unter [Ubuntu](../glossar.md#ubuntu) drücken Sie `Strg+Alt+T`, um das [Terminal](../glossar.md#terminal) zu öffnen. Sie können auch im Anwendungsmenü nach „Terminal" suchen.

**Windows-Analogie:** Wie das Öffnen der Eingabeaufforderung, aber Sie werden es unter Linux viel häufiger verwenden. Das Terminal ist Ihr Hauptwerkzeug für die Interaktion mit dem System.

Wenn Sie das Terminal öffnen, sehen Sie eine Eingabeaufforderung wie:

```
benutzer@rechnername:~$
```

Hier geben Sie Befehle ein. Die Eingabeaufforderung zeigt Ihren Benutzernamen, Computernamen und das aktuelle Verzeichnis.

---

## Durch das Dateisystem navigieren

Linux organisiert Dateien in einer Baumstruktur. Es gibt keine Laufwerksbuchstaben (C:, D:). Alles beginnt bei `/` (root).

### Befehle:

- `pwd` — Print Working Directory (Arbeitsverzeichnis anzeigen)
	- Zeigt Ihren aktuellen Ort.
	- Beispiel:
		```
		$ pwd
		/home/benutzer
		```
- `cd` — Change Directory (Verzeichnis wechseln)
	- Wechseln Sie zwischen Ordnern.
	- Beispiel:
		```
		$ cd /etc
		$ pwd
		/etc
		```
- `ls` — Dateien auflisten
	- Zeigt Dateien und Ordner an.
	- Beispiel:
		```
		$ ls
		Dokumente  Downloads  Bilder
		```
- `cd ..` — Eine Ebene nach oben gehen
	- Beispiel:
		```
		$ cd ..
		```
- `cd ~` — Zu Ihrem Home-Verzeichnis gehen
	- Beispiel:
		```
		$ cd ~
		```

**Tipp:** Verwenden Sie Tab zum automatischen Vervollständigen von Ordnernamen.

---

## Dateien und Verzeichnisse auflisten

- `ls -l` — Langes Format (zeigt Berechtigungen, Eigentümer, Größe, Datum)
	- Beispiel:
		```
		$ ls -l
		-rw-r--r-- 1 benutzer benutzer 4096 Feb 14 10:00 datei.txt
		```
- `ls -a` — Versteckte Dateien anzeigen (Dateien beginnend mit `.`)
- `tree` — Verzeichnisstruktur als Baum anzeigen
	- Beispiel:
		```
		$ tree
		.
		├── Dokumente
		├── Downloads
		└── Bilder
		```

**Windows-Analogie:** Wie das Öffnen von Datei-Explorer, aber Sie sehen alles als Text.

---

## Dateien anzeigen und bearbeiten

- `cat` — Dateiinhalte anzeigen
	- Beispiel:
		```
		$ cat datei.txt
		Hallo, Welt!
		```
- `less` — Datei anzeigen, nach oben/unten scrollen
	- Beispiel:
		```
		$ less datei.txt
		```
- `head` — Erste Zeilen anzeigen
	- Beispiel:
		```
		$ head datei.txt
		```
- `tail` — Letzte Zeilen anzeigen
	- Beispiel:
		```
		$ tail datei.txt
		```
- `nano` — Einfacher Text-Editor
	- Beispiel:
		```
		$ nano datei.txt
		```

**Tipp:** Nano ist anfängerfreundlich. Verwenden Sie Strg+O zum Speichern, Strg+X zum Beenden.

---

## Nach Dateien und Text suchen

- `grep` — Nach Text in Dateien suchen
	- Beispiel:
		```
		$ grep hallo datei.txt
		```
- `find` — Dateien suchen
	- Beispiel:
		```
		$ find . -name "*.txt"
		```
- `locate` — Dateien schnell finden (verwendet eine Datenbank)
	- Beispiel:
		```
		$ locate datei.txt
		```

**Windows-Analogie:** Wie die Verwendung der Windows-Suche oder Strg+F im Notepad.

---

## Programme ausführen

- Geben Sie einfach den Programmnamen ein:
	- Beispiel:
		```
		$ firefox
		```
- Fügen Sie `&` hinzu, um im Hintergrund auszuführen:
	- Beispiel:
		```
		$ firefox &
		```
- Skripte ausführen:
	- Beispiel:
		```
		$ ./meinskript.sh
		```

**Tipp:** Verwenden Sie `chmod +x meinskript.sh`, um ein Skript ausführbar zu machen.

---

## Prozesse verwalten

- `ps` — Laufende Prozesse auflisten
	- Beispiel:
		```
		$ ps
		```
- `top` — Echtzeit-Prozess-Monitor
	- Beispiel:
		```
		$ top
		```
- `htop` — Verbesserter Prozess-Monitor (installieren mit `sudo apt install htop`)
- `kill` — Prozess stoppen
	- Beispiel:
		```
		$ kill 1234
		```
- `systemctl status` — Service-Status überprüfen
	- Beispiel:
		```
		$ systemctl status ssh
		```

**Windows-Analogie:** Wie Task-Manager, aber in Text.

---

## Software installieren

- `apt` — Paketmanager (wie Windows Store oder Installer)
	- Paketliste aktualisieren:
		```
		$ sudo apt update
		```
	- Installierte Pakete aktualisieren:
		```
		$ sudo apt upgrade
		```
	- Ein Programm installieren:
		```
		$ sudo apt install firefox
		```
	- Ein Programm entfernen:
		```
		$ sudo apt remove firefox
		```
- `snap` — Ein weiterer Paketmanager (für einige Apps)

**Tipp:** Verwenden Sie `sudo` für Admin-Aufgaben (wie „Als Administrator ausführen" unter Windows).

---

## Ihre Shell anpassen

- `alias` — Shortcuts für Befehle erstellen
	- Beispiel:
		```
		$ alias ll='ls -l'
		```
- `export` — Umgebungsvariablen setzen
	- Beispiel:
		```
		$ export PATH=$PATH:/mein/benutzerdefinierten/pfad
		```
- Bearbeiten Sie Ihre `.bashrc` Datei, um Änderungen dauerhaft zu machen.

---

## Scripting-Grundlagen

Shell-Skripte ermöglichen es Ihnen, Aufgaben zu automatisieren. Sie sind wie Batch-Dateien unter Windows, aber leistungsfähiger.

**Beispiel-Skript:**

Erstellen Sie eine Datei namens `backup.sh`:

```
#!/bin/bash
tar -czvf backup.tar.gz /home/benutzer/Dokumente
```

Machen Sie es ausführbar:

```
$ chmod +x backup.sh
```

Führen Sie es aus:

```
$ ./backup.sh
```

**Variablen:**

```
name="Welt"
echo "Hallo, $name!"
```

**Argumente:**

```
echo "Erstes Argument: $1"
```

**If-Anweisungen:**

```
if [ "$name" = "Welt" ]; then
	echo "Hallo, Welt!"
fi
```

**Schleifen:**

```
for datei in *.txt; do
	echo $datei
done
```

---

## Produktivitätstipps

- **Tab-Vervollständigung:** Drücken Sie Tab, um Befehle und Dateinamen zu vervollständigen.
- **Befehlsverlauf:** Verwenden Sie die Pfeile nach oben/unten, um vorherige Befehle zu wiederholen.
- **Strg+C:** Führenden Befehl stoppen.
- **Strg+L:** Bildschirm löschen.
- **Strg+R:** Befehlsverlauf durchsuchen.
- **Pipes (`|`):** Ausgabe von einem Befehl an einen anderen senden.
	- Beispiel:
		```
		$ ls | grep txt
		```
- **Umleitung (`>`, `>>`):** Ausgabe in eine Datei speichern.
	- Beispiel:
		```
		$ ls > dateien.txt
		```
- **Absolute vs. relative Pfade:** `/home/benutzer/datei.txt` ist absolut; `datei.txt` ist relativ.
- **Versteckte Dateien:** Dateien beginnend mit `.` sind versteckt. Verwenden Sie `ls -a`, um sie zu sehen.
- **Exit-Codes:** Jeder Befehl gibt einen Code zurück. `echo $?` zeigt das Ergebnis des letzten Befehls (0 = Erfolg).
- **man pages:** Handbücher für Befehle. Versuchen Sie `man ls`.

---

## Fehlersuche

- **Befehl nicht gefunden:** Überprüfen Sie die Schreibweise oder installieren Sie das Programm.
- **Zugriff verweigert:** Verwenden Sie `sudo` oder überprüfen Sie Dateiberechtigungen.
- **Datei nicht gefunden:** Überprüfen Sie Ihr aktuelles Verzeichnis mit `pwd`.
- **Hilfe:** Die meisten Befehle unterstützen `--help`.
	- Beispiel:
		```
		$ ls --help
		```
- **Handbuchseiten:**
	- Beispiel:
		```
		$ man ls
		```
