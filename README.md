# FrameBoost Horizon 26.3

Ein Horizon-Native-Mod mit drei Grafikprofilen für mehr Bildrate. Es ist kein Sodium-JAR und kein Java-Mod.

## Profile

- **Leistung (8 Chunks):** Horizon-Leistungsprofil, 8 Chunks Sichtweite, 6 Chunks Simulationsweite, LOD aus, VSync aus, unbegrenzte Bildrate und Energiesparmodus aus.
- **Maximale FPS (6 Chunks):** wie Leistung, aber 6 Chunks Sichtweite und 5 Chunks Simulationsweite. Dadurch sieht man weniger weit und weiter entfernte Bereiche werden weniger oft simuliert.
- **Ausgewogen (12 Chunks):** Horizon-Ausgewogen-Profil mit 12 Chunks Sichtweite und LOD aus.

Die Profile ändern und speichern die Grafikoptionen der gerade gestarteten Horizon-Instanz. Das Ausgewogen-Profil setzt die Grafikqualität zurück, stellt aber die vorherigen Werte für VSync, FPS-Limit und Energiesparmodus nicht wieder her. Diese kannst du in den Video- und Horizon-Grafikeinstellungen selbst einstellen.

## Installation

In Horizon unter **Mods > Native Mod von GitHub hinzufügen** den [Release v0.1.0](https://github.com/jannnnnnnnnnnik/frameboost-horizon/releases/tag/v0.1.0) eintragen.

Alternativ enthält dieser Quellcode den Horizon-Mixins-Ordner `mixins/frameboost_horizon` mit `horizon-mixin.toml`. Baue ihn mit einer Horizon-Launcher-Version, die Native Mods unterstützt:

```powershell
horizon-launcher native-build "<PFAD_ZUM_PROJEKT>\mixins\frameboost_horizon" --trust-native-code
```

Der Launcher erstellt eine eigene Instanz **Native mods**. Starte diese Instanz und öffne **Optionen > Mods > FrameBoost Horizon > Einstellungen**. Native Mods laufen mit deinen Windows-Benutzerrechten. Lies den Quellcode und bestätige die Vertrauensabfrage nur, wenn du dem Mod vertraust.

## Grenzen

Horizon führt keine Fabric-/Java-Mods wie Sodium aus. Dieses Mod nutzt Horizons eigene Performance-Einstellungen, statt den Renderer durch Sodium zu ersetzen. Es verringert die Grafiklast; der tatsächliche FPS-Unterschied hängt von PC, Welt und Engpass ab und ist nicht garantiert.
