# 07 - Shell-Scripting-Grundlagen

[Shell](../glossar.md#shell)-Skripte sind Textdateien, die [Bash](../glossar.md#bash)-Befehle enthalten — sie werden der Reihe nach ausgeführt, als würdest du jeden Befehl selbst eintippen. Damit kannst du wiederkehrende Aufgaben automatisieren, ohne sie jedes Mal von Hand zu erledigen.

**Windows-Analogie:** Shell-Skripte entsprechen Batch-Dateien (`.bat`) oder PowerShell-Skripten — aber sie sind deutlich mächtiger und auf Linux-Servern allgegenwärtig.

---

## 1. Der Shebang (`#!/bin/bash`)

Die allererste Zeile eines Shell-Skripts heißt **Shebang**. Sie teilt dem System mit, welches Programm das Skript ausführen soll.

```bash
#!/bin/bash
```

Ohne diese Zeile weiß das System nicht, dass es sich um ein Bash-Skript handelt.

**Beispiel — minimales Skript:**

```bash
#!/bin/bash
echo "Hallo, Welt!"
```

Speichere diese Datei als `hallo.sh`. Die Endung `.sh` ist eine Konvention — technisch notwendig ist sie nicht, aber sie macht den Zweck der Datei sofort klar.

---

## 2. Scripts ausführbar machen

Eine neu erstellte Datei kann noch nicht direkt ausgeführt werden. Du musst ihr zuerst die Ausführungsberechtigung geben:

```bash
chmod +x hallo.sh
```

Danach kannst du das Skript starten:

```bash
./hallo.sh
```

Das `./` bedeutet: "Führe die Datei im aktuellen Verzeichnis aus."

**Typischer Arbeitsablauf:**

```bash
nano meinskript.sh      # Skript schreiben
chmod +x meinskript.sh  # Ausführbar machen
./meinskript.sh         # Ausführen
```

---

## 3. Variablen

Variablen speichern Werte, die du später im Skript wiederverwenden kannst. Es gibt kein Schlüsselwort wie `let` oder `var` — du weist einfach direkt zu.

**Wichtig:** Kein Leerzeichen vor oder nach dem `=`.

```bash
#!/bin/bash
name="Alice"
alter=30
echo "Name: $name"
echo "Alter: $alter"
```

Ausgabe:
```
Name: Alice
Alter: 30
```

Variablennamen mit Leerzeichen oder Sonderzeichen immer in Anführungszeichen setzen:

```bash
verzeichnis="/home/alice/Dokumente"
echo "Sicherung von: $verzeichnis"
```

**Windows-Analogie:** Wie Variablen in Batch-Dateien (`set NAME=Alice`), aber mit `$` beim Lesen des Werts.

---

## 4. Argumente: `$1`, `$2`, ...

Wenn du einem Skript beim Aufruf Werte mitgibst (sogenannte **Argumente**), sind diese innerhalb des Skripts über `$1`, `$2` usw. abrufbar.

```bash
#!/bin/bash
echo "Erstes Argument:  $1"
echo "Zweites Argument: $2"
```

Aufruf:

```bash
./skript.sh Hallo Welt
```

Ausgabe:
```
Erstes Argument:  Hallo
Zweites Argument: Welt
```

Weitere nützliche Sondervariablen:

| Variable | Bedeutung                              |
|----------|----------------------------------------|
| `$0`     | Name des Skripts selbst               |
| `$1`-`$9`| Das 1. bis 9. Argument               |
| `$#`     | Anzahl der übergebenen Argumente      |
| `$@`     | Alle Argumente                        |

**Beispiel — Begrüßungsskript:**

```bash
#!/bin/bash
echo "Hallo, $1! Du hast $# Argument(e) übergeben."
```

```bash
./begruessung.sh Maria
```

Ausgabe:
```
Hallo, Maria! Du hast 1 Argument(e) übergeben.
```

---

## 5. if — Bedingungen

Mit `if` kannst du entscheiden, ob ein Codeblock ausgeführt wird oder nicht.

**Grundstruktur:**

```bash
if [ BEDINGUNG ]; then
    # wird ausgeführt, wenn BEDINGUNG wahr ist
fi
```

**Beispiel — Text vergleichen:**

```bash
#!/bin/bash
benutzer="alice"

if [ "$benutzer" = "alice" ]; then
    echo "Willkommen, Alice!"
else
    echo "Unbekannter Benutzer."
fi
```

**Beispiel — Zahl vergleichen:**

```bash
#!/bin/bash
anzahl=5

if [ $anzahl -gt 3 ]; then
    echo "Mehr als 3."
fi
```

Häufige Vergleichsoperatoren:

| Operator | Bedeutung              |
|----------|------------------------|
| `=`      | Gleich (Text)          |
| `!=`     | Ungleich (Text)        |
| `-eq`    | Gleich (Zahl)          |
| `-ne`    | Ungleich (Zahl)        |
| `-gt`    | Größer als             |
| `-lt`    | Kleiner als            |
| `-ge`    | Größer oder gleich     |
| `-le`    | Kleiner oder gleich    |
| `-f`     | Ist eine Datei         |
| `-d`     | Ist ein Verzeichnis    |

**Beispiel — Datei prüfen:**

```bash
#!/bin/bash
if [ -f "/etc/hosts" ]; then
    echo "Die Datei existiert."
else
    echo "Datei nicht gefunden."
fi
```

---

## 6. for — Schleifen

Mit `for` kannst du eine Gruppe von Elementen der Reihe nach abarbeiten.

**Beispiel — Liste durchgehen:**

```bash
#!/bin/bash
for frucht in Apfel Birne Kirsche; do
    echo "Frucht: $frucht"
done
```

Ausgabe:
```
Frucht: Apfel
Frucht: Birne
Frucht: Kirsche
```

**Beispiel — Dateien verarbeiten:**

```bash
#!/bin/bash
for datei in *.txt; do
    echo "Verarbeite: $datei"
done
```

Dieses Skript geht alle `.txt`-Dateien im aktuellen Verzeichnis durch.

**Beispiel — Zahlen durchzählen:**

```bash
#!/bin/bash
for i in 1 2 3 4 5; do
    echo "Schritt $i"
done
```

Oder mit einem Bereich:

```bash
#!/bin/bash
for i in $(seq 1 5); do
    echo "Schritt $i"
done
```

---

## 7. Exit-Codes

Jeder Befehl und jedes Skript gibt beim Beenden einen **Exit-Code** zurück:

- `0` = Erfolg
- Alles andere (1, 2, ...) = Fehler

Du kannst den Exit-Code des zuletzt ausgeführten Befehls mit `$?` abfragen:

```bash
ls /etc/hosts
echo "Exit-Code: $?"   # 0, weil die Datei existiert

ls /gibts/nicht
echo "Exit-Code: $?"   # 2, weil der Pfad nicht existiert
```

In eigenen Skripten kannst du mit `exit` einen Code zurückgeben:

```bash
#!/bin/bash
if [ ! -f "$1" ]; then
    echo "Fehler: Datei nicht gefunden."
    exit 1
fi

echo "Datei gefunden: $1"
exit 0
```

**Warum ist das wichtig?** Andere Skripte oder Tools werten Exit-Codes aus, um zu wissen, ob ein Befehl erfolgreich war. Zum Beispiel kann `cron` (der Aufgabenplaner) reagieren, wenn ein Skript mit einem Fehler endet.

---

## 8. Einfaches Backup-Skript

Alles zusammen in einem praktischen Beispiel: ein Skript, das einen Ordner als komprimiertes Archiv sichert.

```bash
#!/bin/bash

# Zielordner als erstes Argument übergeben, Standardwert: ~/Dokumente
QUELLE="${1:-$HOME/Dokumente}"
ZIEL="$HOME/backups"
DATUM=$(date +%Y-%m-%d_%H-%M-%S)
DATEINAME="backup_$DATUM.tar.gz"

# Backup-Verzeichnis erstellen, falls es nicht existiert
if [ ! -d "$ZIEL" ]; then
    mkdir -p "$ZIEL"
    echo "Verzeichnis erstellt: $ZIEL"
fi

# Prüfen, ob Quellordner existiert
if [ ! -d "$QUELLE" ]; then
    echo "Fehler: Quellordner '$QUELLE' nicht gefunden."
    exit 1
fi

# Backup erstellen
echo "Sichere '$QUELLE' nach '$ZIEL/$DATEINAME' ..."
tar -czf "$ZIEL/$DATEINAME" "$QUELLE"

if [ $? -eq 0 ]; then
    echo "Backup erfolgreich: $DATEINAME"
else
    echo "Fehler beim Erstellen des Backups."
    exit 1
fi

exit 0
```

**Speichern, ausführbar machen und ausführen:**

```bash
nano backup.sh
chmod +x backup.sh
./backup.sh                         # sichert ~/Dokumente
./backup.sh /home/alice/Projekte    # sichert einen anderen Ordner
```

**Was das Skript macht, Schritt für Schritt:**

1. Nimmt einen optionalen Quellordner als Argument (`$1`) — Standard ist `~/Dokumente`.
2. Erstellt einen Zeitstempel für den Dateinamen.
3. Legt das Backup-Verzeichnis an, falls es noch nicht existiert.
4. Prüft, ob der Quellordner vorhanden ist — bricht mit Fehlercode `1` ab, wenn nicht.
5. Erstellt ein komprimiertes Archiv mit `tar`.
6. Prüft den Exit-Code von `tar` und meldet Erfolg oder Fehler.

---

## Häufige Fehler

- **Kein Leerzeichen um `=` bei Variablen:**
  - Falsch: `name = "Alice"` — Bash interpretiert das als Befehl.
  - Richtig: `name="Alice"`

- **Anführungszeichen vergessen:**
  - Falsch: `if [ $name = Alice ]` — bricht bei Leerzeichen im Wert.
  - Richtig: `if [ "$name" = "Alice" ]`

- **Shebang vergessen:**
  - Ohne `#!/bin/bash` kann das Skript mit der falschen Shell ausgeführt werden.

- **Kein `chmod +x`:**
  - `bash: ./skript.sh: Permission denied` — Ausführungsberechtigung fehlt.

---

## Nützliche Befehle beim Skript-Schreiben

| Befehl / Konzept       | Bedeutung                                      |
|------------------------|------------------------------------------------|
| `echo "Text"`          | Text ausgeben                                  |
| `read variable`        | Benutzereingabe einlesen                       |
| `$(befehl)`            | Ausgabe eines Befehls als Wert verwenden       |
| `date +%Y-%m-%d`       | Aktuelles Datum als Text                       |
| `mkdir -p pfad`        | Verzeichnis erstellen (auch verschachtelt)     |
| `tar -czf archiv.tar.gz ordner` | Ordner komprimiert archivieren        |
| `exit 0` / `exit 1`   | Skript mit Erfolg / Fehler beenden             |
| `# Kommentar`          | Zeile wird nicht ausgeführt (Kommentar)        |

---

## Trainingsübungen

1. Schreibe ein Skript, das deinen Benutzernamen und das aktuelle Datum ausgibt.
2. Schreibe ein Skript, das ein Argument entgegennimmt und prüft, ob die angegebene Datei existiert.
3. Schreibe eine `for`-Schleife, die die Zahlen 1 bis 10 ausgibt.
4. Erweitere das Backup-Skript so, dass alte Backups (älter als 7 Tage) automatisch gelöscht werden.
5. Schreibe ein Skript mit einem `if`/`else`, das prüft, ob du root bist (`[ "$EUID" -eq 0 ]`).

---

## Weitere Ressourcen

- [Bash Guide for Beginners (tldp.org)](https://tldp.org/LDP/Bash-Beginners-Guide/html/)
- [Explainshell](https://explainshell.com/) — erklärt jeden Teil eines Befehls
- [ShellCheck](https://www.shellcheck.net/) — prüft Skripte auf häufige Fehler
- [Ubuntu Dokumentation](https://help.ubuntu.com/)
