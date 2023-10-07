# felicity

WIP

- [x] Render moods list
- [x] Ability to enter moods in UI
- [ ] Mood 'calendar' view
    - Clicking the day should toggle the list below the calendar month
- [ ] Make it a menubar app
    - The meubar icon can change over time to remind me to register the current mood. As well as show the general 'climate' of the mood of the day/week.

Before public release,

- [ ] Auto-create `~/.felicity.db` if it does not exist

## Getting Starred

In the `nix develop` shell, run:

```
# Run these in two separate terminals
just tw     # Tailwind watcher
just watch  # Dioxus watcher
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
