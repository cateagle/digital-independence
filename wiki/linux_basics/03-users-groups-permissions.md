# Benutzer, Gruppen und Berechtigungen - Spickzettel

## 1. Root-Berechtigungen
- **Root** ist der Superuser—kann alles machen, auch das ganze System löschen.
- Verwende immer einen normalen Benutzer für alltägliche Aufgaben. Nutze Root (über `sudo`) nur für Admin-Aktionen.
- Root auszuführen ist wie alles als Administrator unter Windows zu starten. Mache das nicht, es sei denn, es ist notwendig.

## 2. Das Sudo-Modell
- `sudo <command>`: Führe einen Befehl als Root (Superuser) aus.
- `sudo -i`: Starte eine Root-Shell (Vorsicht!).
- `sudo su -`: Wechsle zum Root-Benutzer.
- Nur Benutzer in der `sudo`-Gruppe können sudo verwenden.
- [Sudo](../glossar.md#sudo-substitute-user-do) protokolliert alle Aktionen für die Kontrolle.

**Beispiel:**
```bash
sudo apt update
```

## 3. Benutzer
- Jede Person oder jeder Dienst bekommt ein Benutzerkonto.
- Home-Verzeichnisse: `/home/alice`, `/home/bob`
- Dienst-Benutzer: `www-data` (Webserver), `docker` (Container-Engine)

**Benutzer erstellen:**
```bash
sudo adduser alice
```

## 4. Gruppen
- Gruppen ermöglichen es, Berechtigungen zwischen Benutzern zu teilen.
- Beispiel: `sudo`-Gruppe gibt Admin-Rechte.
- Benutzer zu einer Gruppe hinzufügen:
```bash
sudo usermod -aG sudo alice
```
- Gruppen auflisten:
```bash
groups alice
```

## 5. Dateieigentümer und Berechtigungen
Jede Datei hat:
- Einen Eigentümer (Benutzer)
- Eine Gruppe
- Berechtigungsbits (rwx)

**Dateidetails anzeigen:**
```bash
ls -l myfile.txt
```
Ausgabe:
```
-rw-r--r-- 1 alice staff 1234 Feb 14 12:00 myfile.txt
```
- Eigentümer: alice
- Gruppe: staff
- Berechtigungen: rw- (Eigentümer), r-- (Gruppe), r-- (Andere)

## 6. Berechtigungsbits (rwx)
| Symbol | Bedeutung |
|--------|-----------|
| r      | Lesen     |
| w      | Schreiben |
| x      | Ausführen |

Drei Sets:
- Benutzer (Eigentümer)
- Gruppe
- Andere

**Beispiel:**
```
-rwxr-xr--
```
- Benutzer: rwx (Lesen/Schreiben/Ausführen)
- Gruppe: r-x (Lesen/Ausführen)
- Andere: r-- (Lesen)

## 7. Numerische Berechtigungen
Berechtigungen können mit Zahlen gesetzt werden:
- 7 = rwx
- 6 = rw-
- 5 = r-x
- 4 = r--

**Beispiel:**
```bash
chmod 755 myscript.sh
```
- 7 (Benutzer): rwx
- 5 (Gruppe): r-x
- 5 (Andere): r-x

**Häufige Werte:**
- 755: Skripte/Programme (Benutzer kann bearbeiten/ausführen, Andere können ausführen)
- 644: Dokumente (Benutzer kann bearbeiten, Andere können lesen)

## 8. Berechtigungen und Eigentümer ändern
- `chmod <mode> <file>`: Berechtigungen ändern
- `chown <user>:<group> <file>`: Eigentümer/Gruppe ändern

**Beispiele:**
```bash
chmod 600 secrets.txt   # Nur Eigentümer kann lesen/schreiben
chown bob:staff report.txt
```

## 9. Dienst-Benutzer
- Spezielle Benutzer zum Ausführen von Diensten (z.B. Webserver, Datenbank)
- Begrenzte Berechtigungen aus Sicherheitsgründen
- Melde dich nie als diese Benutzer an

## 10. Nützliche Befehle
| Befehl | Was es macht |
|--------|--------------|
| adduser <name> | Erstelle einen neuen Benutzer |
| usermod -aG <group> <user> | Benutzer zu Gruppe hinzufügen |
| passwd <user> | Benutzerpasswort ändern |
| groups | Zeige deine Gruppen |
| chmod <mode> <file> | Berechtigungen ändern |
| chown <user>:<group> <file> | Eigentümer/Gruppe ändern |

## 11. Praktische Beispiele
**Benutzer hinzufügen:**
```bash
sudo adduser alice
```
**Zu Gruppe hinzufügen:**
```bash
sudo usermod -aG sudo alice
```
**Dateiberechtigungen ändern:**
```bash
chmod 644 myfile.txt
```
**Dateieigentümer ändern:**
```bash
chown alice:staff myfile.txt
```

## 12. Berechtigungsdiagramme

```
Berechtigungen: rwxr-xr--

Benutzer (Eigentümer): rwx
Gruppe:               r-x
Andere:               r--

Numerisch: 7 5 4
```

## 13. Troubleshooting und Tipps
- Wenn du nicht auf eine Datei zugreifen kannst, überprüfe die Berechtigungen mit `ls -l`.
- Verwende `sudo`, wenn du "Permission denied" erhältst.
- Verwende Root nie für reguläre Aufgaben.
- Verwende Gruppen zur Verwaltung des Zugriffs.
- Verwende `chmod` und `chown` vorsichtig—Fehler können dich aussperren!
- Wenn du die Berechtigungen vermesst, stelle sie wieder her mit:
```bash
sudo chown $USER:$USER <file>
sudo chmod 644 <file>
```

## 14. Windows vs. Linux Berechtigungen
- Windows: Berechtigungen werden über GUI gesetzt, oft verwirrend.
- Linux: Berechtigungen sind klar, werden über Kommandozeile gesetzt.
- Keine "Everyone"-Gruppe—verwende "Andere" für öffentlichen Zugriff.
- Keine "versteckten" Admin-Konten—Root ist immer sichtbar.

## 15. Erweiterte Themen (Optional)
- [ACLs](../glossar.md#acl-access-control-list) (Access Control Lists): Granularere Berechtigungen.
- `getfacl`, `setfacl`: Verwalte ACLs.
- Sticky Bit, setuid, setgid: Spezielle Berechtigungsbits.

## 16. Referenztabelle
| Symbol | Numerisch | Bedeutung       |
|--------|-----------|-----------------|
| rwx    | 7         | Lesen/Schreiben/Ausführen |
| rw-    | 6         | Lesen/Schreiben |
| r-x    | 5         | Lesen/Ausführen |
| r--    | 4         | Lesen           |
| ---    | 0         | Keine           |

## 17. Nützliche Links
- [Linux-Dateiberechtigungen erklärt](https://www.linux.com/training-tutorials/understanding-linux-file-permissions/)
- [Offizieller Ubuntu-Leitfaden zur Benutzerverwaltung](https://help.ubuntu.com/community/AddUsersHowTo)
- [Linux-Kommandozeilen-Grundlagen](https://ubuntu.com/tutorials/command-line-for-beginners)
