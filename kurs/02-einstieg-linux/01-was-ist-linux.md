# Was ist Linux?

Linux begegnet dir überall: auf Webservern, in Smartphones (Android), in Routern, auf Supercomputern und in der Cloud. Aber was genau ist Linux eigentlich?

---

## Der Kernel: Das Herzstück

Linux ist im strengen Sinne nur ein **[Kernel](../../wiki/glossar.md#kernel)**, die unterste Schicht eines Betriebssystems, die direkt mit der Hardware kommuniziert. Der Kernel verwaltet:

- den **Arbeitsspeicher** (wer bekommt wie viel RAM?)
- die **CPU** (welches Programm läuft wann?)
- **Geräte** wie Festplatten, Netzwerkkarten und USB-Geräte
- die **Sicherheit** (welcher Prozess darf was tun?)

**Analogie:** Der Kernel ist wie das Fundament eines Hauses. Du siehst es nicht, aber ohne es steht nichts.

```
Anwendungen (Firefox, VS Code, dein Server-Programm)
      ↓
Shell / Terminal (bash, zsh)
      ↓
Betriebssystem (Bibliotheken, Dienste)
      ↓
Kernel (Linux)
      ↓
Hardware (CPU, RAM, Festplatte, Netzwerk)
```

---

## Geschichte: Von Linus zu Linux

1991 veröffentlichte der finnische Student **Linus Torvalds** den ersten Linux-Kernel als freies Hobby-Projekt. Heute arbeiten tausende Entwickler aus aller Welt daran mit – darunter Ingenieure von Google, Red Hat, Intel und Microsoft.

Der Name "Linux" setzt sich zusammen aus **Li**nus und U**nix**. [Unix](../../wiki/glossar.md#unix) war das professionelle Betriebssystem, das Linux ursprünglich inspirierte.

---

## Open Source: Jeder kann reinschauen

Linux ist **Open Source**. Der gesamte Quellcode ist öffentlich einsehbar und frei verwendbar. Das hat mehrere praktische Vorteile:

| Vorteil | Bedeutung |
|---|---|
| Kostenlos | Keine Lizenzkosten, auch für kommerzielle Nutzung |
| Transparent | Jeder kann den Code prüfen – keine versteckten Hintertüren |
| Anpassbar | Du kannst Linux nach deinen Bedürfnissen verändern |
| Community | Millionen von Entwicklern beheben Fehler und fügen Funktionen hinzu |

**Windows-Analogie:** Windows ist wie ein verschlossenes Auto – du kannst es fahren, aber nicht unter die Motorhaube schauen. Linux ist wie ein Bausatz mit vollständigem Bauplan.

---

## Warum Linux für Server?

Wenn du selbst hostest, wirst du fast immer mit Linux arbeiten. Das hat gute Gründe:

- **Stabilität**: Linux-Server laufen oft jahrelang ohne Neustart
- **Ressourcenschonend**: Läuft auch auf günstiger Hardware und VPS-Instanzen mit wenig RAM
- **Sicherheit**: Klares Rechtemodell, regelmäßige Sicherheitsupdates, keine unnötige Software
- **Kontrolle**: Du entscheidest, was auf dem System läuft – keine automatischen Updates ohne dein Wissen
- **Standard**: Fast alle Server-Software ([nginx](../../wiki/glossar.md#nginx-engine-x), [PostgreSQL](../../wiki/glossar.md#postgresql), [Docker](../../wiki/glossar.md#docker)) ist primär für Linux entwickelt
- **Kosten**: Ein Linux Server ist quasi kostenlos. Du musst dich nicht um Lizenzkeys kümmern, wie es z.B. bei einem Windows Server der fall wäre.

---

## Linux vs. Windows: Ein Vergleich

| Merkmal         | Linux                           | Windows                    |
|-----------------|---------------------------------|----------------------------|
| Kosten          | Kostenlos                       | Lizenzpflichtig            |
| Oberfläche      | Meist [Terminal](../../wiki/glossar.md#terminal), optional [GUI](../../wiki/glossar.md#gui-graphical-user-interface)    | GUI als Standard           |
| Paketverwaltung | `apt`, `dnf`, `pacman`          | Microsoft Store, manuell   |
| Dateisystem     | `/` als Wurzel                  | `C:\`, `D:\` als Laufwerke |
| Berechtigungen  | Benutzer/Gruppe/Andere          | ACLs                       |
| Server-Nutzung  | Dominierend (~96% aller Server) | Nische                     |

---

## Tipp

Du musst kein Linux-Experte sein, bevor du anfängst. Die meisten Self-Hosting-Aufgaben bestehen aus wenigen, sich wiederholenden Befehlen. Mit der Zeit wächst dein Wissen ganz von selbst.
