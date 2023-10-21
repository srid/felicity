# felicity

WIP

- [x] Render moods list
- [x] Ability to enter moods in UI
- [ ] Mood 'calendar' view
    - Clicking the day should toggle the list below the calendar month
- [ ] Make it a menubar app
    - The meubar icon can change over time to remind me to register the current mood. As well as show the general 'climate' of the mood of the day/week.
- [ ] Remindes & Nudgers
    - Figure out UX to nudge the user to enter mood on a consistent basis such that the calendar view statistics are meaningful.

Before public release,

- [x] Auto-create `~/.felicity.db` if it does not exist

## Getting Starred

In the `nix develop` shell, run:

```
just watch
```

### Creating macOS app bundle

```
just bundle
```

### Running via Nix

```
nix run github:srid/felicity
# Or just `nix run` in the project directory
```
