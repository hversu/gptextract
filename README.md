combines `simparse` [1] and `callgpt` [2] to take unstructured data (such as a web page or article) and extract declared entities into nodes (and edges to define relationships between nodes) 

[1] https://github.com/hversu/simparse

[2] https://github.com/hversu/gptcall

## Config

- Requires: OpenAI API Key (update my_secret.rs)

- If you want stealth (proxy) mode:
  1. install/run tor service (linux) (or setup your own proxy)
  2. uncomment `let proxy_url: &str = "socks5h://127.0.0.1:9050";` - edit if using your own proxy
  3. comment out `let proxy_url: Option<&str> = None;`

## Example Usage

example usage:
`cargo run https://news.google.com organization,country,person,demographic,event`

example response:

```
OpenAI Response: {
  "nodes": [
    {"value": "Google", "type": "organization"},
    {"value": "Android", "type": "technology"},
    {"value": "iOS", "type": "technology"},
    {"value": "United States", "type": "country"},
    {"value": "CNN", "type": "organization"},
    {"value": "UK", "type": "country"},
    {"value": "New York Times", "type": "organization"},
    {"value": "Megan Specia", "type": "person"},
    {"value": "Fox News", "type": "organization"},
    {"value": "Scott Mcdonald", "type": "person"},
    {"value": "Yahoo Finance", "type": "organization"},
    {"value": "Jill Lawless", "type": "person"},
    {"value": "Brian Melley", "type": "person"},
    {"value": "Kamala Harris", "type": "person"},
    {"value": "The Washington Post", "type": "organization"},
    {"value": "Matt Viser", "type": "person"},
    {"value": "Democratic Party", "type": "organization"},
    {"value": "Edward-Isaac Dovere", "type": "person"},
    {"value": "ABC", "type": "organization"},
    {"value": "George Stephanopoulos", "type": "person"},
    {"value": "The Hill", "type": "organization"},
    {"value": "Donald Trump", "type": "person"},
    {"value": "Mike Pence", "type": "person"},
    {"value": "Newsweek", "type": "organization"},
    {"value": "Michael M. Grynbaum", "type": "person"},
    {"value": "Biden", "type": "person"},
    {"value": "Hurricane Beryl", "type": "event"}
  ],
  "edges": [
    {"from": "Google", "to": "Android", "type": "provides"},
    {"from": "Google", "to": "iOS", "type": "provides"},
    {"from": "CNN", "to": "UK", "type": "reports"},
    {"from": "New York Times", "to": "Megan Specia", "type": "authors"},
    {"from": "Fox News", "to": "Scott Mcdonald", "type": "reports"},
    {"from": "Yahoo Finance", "to": "Jill Lawless", "type": "reports"},
    {"from": "Yahoo Finance", "to": "Brian Melley", "type": "reports"},
    {"from": "Yahoo Finance", "to": "Kamala Harris", "type": "mentions"},
    {"from": "The Washington Post", "to": "Matt Viser", "type": "reports"},
    {"from": "Democratic Party", "to": "Edward-Isaac Dovere", "type": "mentions"},
    {"from": "ABC", "to": "George Stephanopoulos", "type": "hosts"},
    {"from": "The Hill", "to": "Donald Trump", "type": "mentions"},
    {"from": "Newsweek", "to": "Michael M. Grynbaum", "type": "reports"},
    {"from": "Newsweek", "to": "Donald Trump", "type": "mentions"},
    {"from": "Biden", "to": "Hurricane Beryl", "type": "mentions"}
  ]
}
```
