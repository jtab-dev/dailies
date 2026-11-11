# Risk Mitigation & Technical Constraints

## Core Technical Risks

* **Highest Risk Feature:** Idle-Game TUI

* **The Hazard:** Complex TUI rendering bottleneck, introducing flickering and stutters

## The Pivot Strategy

If our primary implementation for this feature fails or takes too long, our backup plan is to:

1. Having the idle-game's graphics render generatively, to avoid having to re-render the screen.
2. Keeping the idle-game's graphics and UI elements static, to avoid rendering problems.
