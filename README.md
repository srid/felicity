# felicity

Felicity is a extremely simple mood tracker app with the goal of ultimately *becoming useless* to the user. It is a work-in-progress and is **not ready for general use** yet.

The basic idea is to track, as often as possible, whether one is [feeling good](https://srid.ca/feeling-good) or not (nothing more complicated than that) over time, and thereon effectuate [a beneficial modification to one's habitual mood](https://srid.ca/affective-awareness) which modification when perfected (wherein one is feeling good 99% of the time alive) leads to the app becoming useless.

WIP

- [x] Render moods list
- [x] Ability to enter moods in UI
- [ ] Mood 'calendar' view
    - Clicking the day should toggle the list below the calendar month
- [ ] Make it a menubar app
    - The meubar icon can change over time to remind me to register the current mood. As well as show the general 'climate' of the mood of the day/week.
- Recording frequency
    - Must register at least every hour, from ~9am to ~9pm
        - Count the hours which received moods
    - [x] Remindes & Nudgers
        - Figure out UX to nudge the user to enter mood on a consistent basis such that the calendar view statistics are meaningful.
- Mood progression
    1. Record every day at regular frequency (see above)
    1. Achieve days of 100% feeling good
    1. Achieve day *streaks* of feeling good
        - Collapse the view to show streaks as single entry (only expand if registering feeling-bad again)
    1. Achieve a week of 100% feeling good
        - Give 'levels' to such streaks.
- "Why" of feeling bad
    - KISS: Tags (which could be distinct emojis) can be attached to each mood entry. The UI can present a popup dropdown when clicking on a recorded registry. Selecting an emoji can highlight all mood entries with the same tag, along with the tag information.

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

(Broken on macOS: https://github.com/srid/felicity/issues/2)

```
nix run github:srid/felicity
# Or just `nix run` in the project directory
```
