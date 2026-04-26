# Wie verdienen Unternehmen mit Open Source Geld?

"Wenn die Software kostenlos ist – wie soll da jemand Geld verdienen?" Diese Frage
hört man oft. Die Antwort: Es gibt viele funktionierende Geschäftsmodelle rund um
Open Source. Einige der wertvollsten Technologieunternehmen der Welt basieren darauf.

---

## Warum Open Source überhaupt wirtschaftlich sinnvoll ist

Open Source ist kein Widerspruch zu kommerziellem Erfolg. Im Gegenteil:

- **Kostenlose Verbreitung** bedeutet schnelle Adoption – keine Vertriebskosten
- **Community-Beiträge** beschleunigen die Entwicklung enorm
- **Vertrauen durch Transparenz** – Unternehmen setzen lieber auf Software, die sie
  selbst prüfen können
- **Ökosystem-Effekte**: Je mehr Nutzer, desto wertvoller die Plattform

---

## Geschäftsmodell 1: Professional Services (Support & Abonnements)

Das klassischste Modell: Die Software ist kostenlos, professioneller Support kostet.
Unternehmen können mit Open Source Software zwar nicht per se Geld verdienen, aber
sie können jeglichen flankierenden Service kommerzialisieren.

**Wie es funktioniert:**
- Kern-Software ist frei verfügbar (Community Edition)
- Unternehmen zahlen für [SLAs](../../wiki/glossar.md#sla-service-level-agreement), garantierte Antwortzeiten, zertifizierten Support,
  Sicherheits-Backports und Compliance-Dokumentation
- Auch Wartung, Hosting, kundenspezifische Entwicklung, Beratung und Schulung
  können kostenpflichtig angeboten werden

**Beispiele:**

| Unternehmen | Produkt | Modell |
|---|---|---|
| Red Hat (IBM) | RHEL | Abonnement für Support & Zertifizierungen |
| Canonical | Ubuntu | Ubuntu Pro, Landscape, kommerzieller Support |
| Elastic | Elasticsearch | Support-Abos, Elastic Cloud |

**Red Hat** ist das bekannteste Beispiel: Das Unternehmen wurde 2019 für **34 Milliarden
Dollar** von IBM übernommen – mit einem Geschäftsmodell, das vollständig auf Open Source basiert.

> **Hinweis für den Markt:** Insbesondere kleine und mittlere Unternehmen haben oft keine
> eigenen IT-Abteilungen oder Fachkräfte für Open Source Software – sie sind ein wichtiges
> Kundensegment für dieses Modell.

---

## Geschäftsmodell 2: Widget Frosting (Hardware + Software)

Dieses Modell verbindet Open-Source-Software mit kostenpflichtiger Hardware. Die Software
steht unter einer Open Source-Lizenz, wohingegen die dazugehörige Hardware käuflich
erworben werden muss.

**Wie es funktioniert:**
- Software (z.B. Treiber, Steuerungssoftware) ist Open Source
- Einnahmen kommen aus dem Hardware-Verkauf
- Variation: Hardware günstig/kostenlos, Software kostenpflichtig lizenziert

**Beispiele:**

| Branche | Beispiel | Open-Source-Teil | Kostenpflichtig |
|---|---|---|---|
| Drucker | Druckerhersteller | Treiber-Software | Drucker selbst |
| Smart Home | Philips Hue (teilweise) | API & SDK | Hardware-Produkte |
| Maschinenbau | Industrieanlagen | Steuerungs-Software (Basis) | Produktionsmaschinen |

> **Besonders interessant** für industrielle Anwendungen im Maschinenbau: Die Hardware
> muss erworben werden, die Basis-Software steht Open Source zur Verfügung. Das kann
> auch als Open-Core-Variante ausgestaltet sein.

---

## Geschäftsmodell 3: Open Core

Das Kernprodukt ist Open Source, aber bestimmte **Enterprise-Features** sind proprietär
und kosten Geld.

```
Open Source (kostenlos)          Proprietär (kostenpflichtig)
────────────────────────         ──────────────────────────────
Kernfunktionen                   Single Sign-On (SSO)
Community-Features               Erweiterte Benutzerrollen
Basis-API                        Audit-Logs
Self-hosted                      Premium-Support
                                 Compliance-Berichte
```

**Beispiele:**

| Unternehmen | Open-Source-Teil | Proprietärer Teil |
|---|---|---|
| GitLab | GitLab CE | GitLab EE (SSO, SAML, etc.) |
| HashiCorp | Terraform, Vault | Enterprise-Features |
| Confluent | Apache Kafka | Confluent Platform |
| Grafana Labs | Grafana OSS | Grafana Cloud, Enterprise-Plugins |

**Kritik am [Open-Core](../../wiki/glossar.md#open-core)-Modell:** Manche Community-Mitglieder empfinden es als unfair,
wenn die "besten" Features hinter einer Paywall verschwinden. Wichtig ist eine sorgfältige
Abwägung: Der Open-Source-Kern muss auch ohne die proprietären Features einen echten
Mehrwert für die Community bieten.

---

## Geschäftsmodell 4: Hosted Service (SaaS)

Die Software läuft auf den eigenen Servern des Unternehmens als Managed Service.
Kunden zahlen dafür, dass sie sich nicht selbst um Betrieb, Updates und Skalierung
kümmern müssen.

**Das Prinzip:** "Du könntest es selbst hosten – aber wir machen es für dich."

**Beispiele:**

| Anbieter | Produkt | Open-Source-Basis |
|---|---|---|
| MongoDB Inc. | MongoDB Atlas | MongoDB |
| Elastic | Elastic Cloud | Elasticsearch |
| Grafana Labs | Grafana Cloud | Grafana |
| Sentry | sentry.io | Sentry (self-hosted kostenlos) |
| Plausible | plausible.io | Plausible Analytics |

**Das "Amazon-Problem":**
AWS, Google Cloud und Azure haben jahrelang Open-Source-Software kostenlos als Managed
Service angeboten, ohne zur Entwicklung beizutragen. Das hat dazu geführt, dass einige
Unternehmen ihre Lizenz gewechselt haben (siehe unten).

---

## Geschäftsmodell 5: Open APIs

Insbesondere im Rahmen der Plattformökonomie bieten Unternehmen offene Schnittstellen
unter einer Open-Source-Lizenz an. Entwickler können das Angebot für ihre Bedürfnisse
anpassen oder erweitern – und tragen dabei zur Attraktivität des ursprünglichen Produkts bei.

**Wie es funktioniert:**
- Kern-Produkt oder Plattform ist proprietär und kostenpflichtig
- API, SDK oder Entwickler-Tools stehen Open Source zur Verfügung
- Dritten wird ermöglicht, eigene Erweiterungen und Apps zu bauen
- Das Ökosystem wächst, ohne dass das Unternehmen alles selbst entwickeln muss

**Beispiele:**

| Unternehmen | Produkt | Open-Source-Teil | Geschäftsmodell |
|---|---|---|---|
| Philips Hue | Smart-Beleuchtung | Entwickler-API & SDK | Hardware-Verkauf + Ökosystem |
| Twilio | Kommunikationsplattform | SDKs | API-Nutzungsgebühren |
| Stripe | Zahlungsabwicklung | Client-Bibliotheken | Transaktionsgebühren |

**Strategischer Vorteil:**
Unternehmen können durch eine frühe Öffnung der Schnittstelle einen **de-facto-Standard**
in ihrer Branche etablieren. Besonders sinnvoll, wenn ein Produkt heute noch differenzierend
ist, aber langfristig zum "selbstverständlichen Allgemeingut" werden könnte.

---

## Geschäftsmodell 6: Dual Licensing

Die gleiche Software wird unter zwei Lizenzen angeboten:
- **Open Source** für Community/private Nutzung (z.B. [GPL](../../wiki/glossar.md#gpl-gnu-general-public-license))
- **Kommerzielle Lizenz** für Unternehmen, die die GPL-Bedingungen nicht erfüllen können

**Warum das funktioniert:**
GPL zwingt dazu, abgeleitete Werke ebenfalls unter GPL zu veröffentlichen. Unternehmen,
die das nicht wollen (weil sie proprietäre Software bauen), müssen die kommerzielle
Lizenz kaufen.

**Beispiele:**

| Produkt | Open-Source-Lizenz | Kommerzielle Lizenz |
|---|---|---|
| MySQL | GPL | Oracle Commercial License |
| Qt | LGPL/GPL | Qt Commercial |
| MongoDB (alt) | AGPL | Commercial |

---

## Geschäftsmodell 7: Spenden & Stiftungen

Manche Projekte sind nicht gewinnorientiert und finanzieren sich über Spenden,
Mitgliedsbeiträge oder Stiftungsgelder.

**Beispiele:**

| Projekt | Organisation | Finanzierung |
|---|---|---|
| Linux-Kernel | Linux Foundation | Mitgliedsbeiträge von Intel, Google, etc. |
| Firefox | Mozilla Foundation | Lizenzgebühren (Google als Standardsuche) |
| Wikipedia | Wikimedia Foundation | Spenden |
| Python | Python Software Foundation | Spenden, Sponsoring |
| Debian | Software in the Public Interest | Spenden |

---

## Lizenzwechsel: Wenn Open Source enger wird

Einige Unternehmen haben ihre Lizenz nachträglich verschärft, als Cloud-Anbieter zu
viel vom Kuchen abbekamen:

| Unternehmen | Produkt | Vorher | Nachher | Reaktion |
|---|---|---|---|---|
| Elastic | Elasticsearch | Apache 2.0 | [SSPL](../../wiki/glossar.md#sspl-server-side-public-license) / Elastic License | AWS forkt zu OpenSearch |
| HashiCorp | Terraform, Vault | MPL 2.0 | BSL 1.1 | Community forkt zu OpenTofu |
| MongoDB | MongoDB | AGPL | SSPL | Distros entfernen Paket |
| Redis Labs | Redis-Module | Apache 2.0 | RSALv2 | AWS/Google forken |

**Was ist SSPL?**
Die *Server Side Public License* (von MongoDB entwickelt) ist technisch kein
"echter" Open-Source-Standard nach [OSI](../../wiki/glossar.md#osi-open-source-initiative)-Definition – obwohl der Code öffentlich ist.
Sie verlangt, dass alle Dienste, die auf der Software basieren, ebenfalls unter SSPL
veröffentlicht werden – also praktisch unmöglich für Cloud-Anbieter.

---

## Zusammenfassung

```
Open-Source-Geschäftsmodelle
├── Professional Services  → Red Hat, Canonical
├── Widget Frosting        → Druckerhersteller, Maschinenbau
├── Open Core              → GitLab, HashiCorp, Grafana
├── Hosted SaaS            → Elastic Cloud, MongoDB Atlas, Sentry
├── Open APIs              → Philips Hue, Twilio, Stripe
├── Dual Licensing         → MySQL, Qt
└── Spenden/Stiftung       → Linux, Firefox, Python
```

Für dich als Self-Hoster bedeutet das: Du profitierst von professionell entwickelter,
gut gewarteter Software – kostenlos. Die Unternehmen dahinter sind nicht altruistisch,
aber ihre Interessen decken sich oft mit deinen.

---

## Quellen

- Culotta, Carina; Duparc, Estelle (2022): **Open-Source Strategies for Companies – Insights and Guidance.**
  Fraunhofer IML Schriftenreihe. Projektteams: DB Schenker (Simjees Abraham, Maik Schmidt, Tilo Wiedera),
  Fraunhofer IML (Carina Culotta, Simon Lechtenberg), Fraunhofer ISST (Philipp Hagenhoff, Anna-Maria Schleimer).
  [https://www.iml.fraunhofer.de/content/dam/iml/de/documents/OE%20250/28_Whitepaper_Opensource.pdf](https://www.iml.fraunhofer.de/content/dam/iml/de/documents/OE%20250/28_Whitepaper_Opensource.pdf)

---

## Weiter geht es

→ [Communities & Gründe](05-communities-und-gruende.md)
