# slack-up
* Makes daily Slack stand-up message easier to do by automating info-gathering from GCalendar + Github
* Also written to help me learn a bit of Rust by following https://rust-lang-nursery.github.io/cli-wg/tutorial/testing.html and playing around.

----
 
### Usage

`$ slack-up` 

Simple as that! The program will pull Calendar info and Github activity info
to build a summary of yesterday and an outline of what you'll be
up to today.

#### Example Output

```bash
*Yesterday*

- Some kind of Calendar event
- Some kind of Github event
- Some kind of Github event related to reviewing code

*Today*

- Pull in outstanding review requests
- Some kind of Calendar event
- Some kind of Calendar event
```

### Feature Wishlist

- Allow for saving the snippets to disk somewhere
- Allow for pushing updates to places like Notion
- Allow for editing a slack-up `slack-up edit <day>` 

### Development

#### Setup

`coming soon`

#### Testing

`cargo test`

