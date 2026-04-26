# 01 - Das Linux-Mentalmodell

Falls du nur Windows verwendet hast, kann sich Linux wie eine völlig andere Welt anfühlen. Dieser Abschnitt erklärt die Kernideen hinter Linux anhand von Windows-Analogien und praktischen Beispielen, um dir zu helfen, zu verstehen, wie Linux funktioniert und warum es so entworfen wurde.

## Alles ist eine Datei

In Windows hast du Dateien, Ordner und spezielle Dinge wie Geräte (Drucker, Festplatten) und Prozesse. In Linux wird fast alles als Datei behandelt:
- Deine Festplatte, dein USB-Stick und sogar deine Maus werden als Dateien in speziellen Ordnern dargestellt.
- Möchtest du sehen, was deine Tastatur tut? Du kannst eine Datei "lesen", die sie darstellt.
- Dies macht Automatisierung und Scripting viel einfacher, da du mit allem auf ähnliche Weise interagierst.

**Windows-Analogie:** Stell dir vor, du könntest Editor öffnen und in eine Datei schreiben, um deinen Drucker zu steuern oder deine Mausbewegungen zu lesen.

## Dateisystem-Hierarchie

Windows verwendet Laufwerksbuchstaben (C:, D:) und Ordner wie Programmdateien, Benutzer und Windows. Linux hat einen einzigen einheitlichen Verzeichnisbaum, der bei `/` beginnt (genannt "root").

Wichtige Verzeichnisse:
- `/etc`: Ähnlich wie die Systemsteuerung oder Registrierung von Windows, aber alle Einstellungen sind in Textdateien gespeichert.
- `/var`: Wo Protokolle und sich ändernde Daten leben (wie Ereignisanzeige-Protokolle).
- `/home`: Wie dein "Benutzer"-Ordner; jeder Benutzer hat seinen eigenen Bereich.
- `/usr`: Wo die meisten Programme und ihre Dateien installiert werden (wie Programmdateien).
- `/opt`: Für optionale oder Software von Drittanbietern.

**Tipp:** Es gibt keine Laufwerksbuchstaben. Alle Laufwerke und Geräte sind "eingebunden" ([mounted](../glossar.md#mount)) irgendwo in diesem Baum.

## Konfiguration lebt in Textdateien

In Windows sind Einstellungen oft in der Registrierung oder hinter [GUIs](../glossar.md#gui-graphical-user-interface) versteckt. In Linux wird fast alle Konfiguration in einfachen Textdateien gespeichert. Du kannst diese Dateien mit jedem Texteditor öffnen, lesen und bearbeiten.

**Beispiel:** Möchtest du ändern, wie dein Netzwerk funktioniert? Bearbeite eine Datei in `/etc`.

**Warum?** Dies macht es einfach, zu automatisieren, zu sichern und Probleme zu beheben. Es werden keine speziellen Werkzeuge benötigt.

## Protokolle leben in /var/log

Windows hat die Ereignisanzeige. Linux speichert Protokolle als Textdateien in `/var/log`.
- Möchtest du sehen, was während des Starts passiert ist? Lese `/var/log/syslog` oder `/var/log/messages`.
- Du kannst Protokolle mit einfachen Befehlen suchen, lesen und analysieren.

## Dienste laufen im Hintergrund

Windows hat "Dienste" (wie Druckspooler, Windows Update). Linux nennt diese "[Dämonen](../glossar.md#daemon)" (Hintergrundprozesse).
- Verwaltet mit Werkzeugen wie `systemctl` (ähnlich dem Windows-Dienste-Manager).
- Beispiele: Webserver, Datenbank, SSH-Server.

**Du kannst diese Dienste über die Befehlszeile starten, stoppen und überprüfen.**

## Root ist allmächtig (und gefährlich)

Windows hat "Administrator"-Konten. Linux hat "root". Root kann alles tun – Dateien löschen, Einstellungen ändern, das System beschädigen.
- Du solltest root nur verwenden, wenn es notwendig ist, um Fehler zu vermeiden.
- Die meisten Befehle werden als normaler Benutzer ausgeführt, und du verwendest `sudo`, um vorübergehend root zu werden ([Sudo](../glossar.md#sudo-substitute-user-do)).

**Tipp:** Alles als root auszuführen ist riskant. Verwende immer nur die minimal erforderlichen Berechtigungen.
