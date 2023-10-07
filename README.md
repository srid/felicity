# felicity

WIP

- [x] Render moods list
- [x] Ability to enter moods in UI
- [ ] Ability to delete entries (useful during app testing)
- [ ] Mood 'calendar' view
    - Clicking the day should toggle the list below the calendar month

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
