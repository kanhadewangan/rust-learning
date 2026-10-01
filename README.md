# hello — Wow, Another Actix Todo Server. How Original. 🙄

> Congratulations. You made the 9,874,321st Todo app. Your parents must be so proud.

![Rust](https://img.shields.io/badge/Rust-Because_JS_was_too_easy-orange) ![Sarcasm](https://img.shields.io/badge/Sarcasm-100%25-critical) ![Production_Ready](https://img.shields.io/badge/Production_Ready-Absolutely_Not-red)

---

### What is this masterpiece? 🎨

Oh, nothing special. Just a groundbreaking, never-seen-before Actix-web server that:

- Lets users `signup` — because the world definitely needed another `/signup` endpoint
- Hashes passwords with `argon2` and a hardcoded salt `randomsalt` — wow, such security, hackers are trembling
- Does Todo CRUD — because if you haven't built a Todo app, are you even a developer?
- Has a `matching()` function that does... something. Nobody knows. Even the compiler is confused.

We were *totally* learning Rust, not just copy-pasting from Stack Overflow.

### Tech Stack — The Usual Suspects 🛠️

- **Rust** — Chose it for the borrow checker trauma. You love pain, we get it.
- **Actix-web 4** — Fastest framework, so your Todo app can handle 10 requests/sec instead of 9
- **Serde** — For JSON, because manually parsing strings is too mainstream
- **Argon2** — Military-grade hashing, paired with a salt a 5-year-old could guess

### Project Structure — Try Not To Get Lost 📁

```
hello/
├── src/
│   ├── main.rs       # Where all routes are duct-taped together
│   ├── users/mod.rs  # Signup logic that prints passwords to console for debugging :)
│   └── todo/mod.rs   # Mutex<Vec<Todos>> — because we love deadlocks
├── Cargo.toml        # 3 dependencies, 300 warnings
└── README.md         # You are here. Wasting time.
```

### API Endpoints — Please Applaud 👏

| Method | Route | Description (since you can't read code) |
|--------|-------|-----------------------------------------|
| `GET` | `/` | Returns `Hello, world!` — The pinnacle of creativity |
| `POST` | `/signup` | Creates a user. Hashes password. Prints it to logs anyway. |
| `POST` | `/api/v1/update` | Updates username. Very useful. Much needed. |
| `POST` | `/api/v1/todo` | Create a Todo. Your 50th today. |
| `GET` | `/api/v1/todo` | Get all Todos. Spoiler: it's an empty Vec. |
| `PUT` | `/api/v1/todo/{id}` | Update a Todo. Because you changed your mind. Again. |
| `DELETE` | `/api/v1/todo/{id}` | Delete a Todo. Like your motivation. |

All stored in `Mutex<Vec<Todos>>` in memory. Restart the server and *poof* — data gone. Who needs a database anyway? Persistence is overrated.

### How To Run This Marvel 🏃‍♂️

```bash
# Step 1: Clone this revolutionary code
git clone <repo-url>
cd hello

# Step 2: Pray that Rust is installed
cargo run

# Step 3: Open http://127.0.0.1:8080 and feel disappointed
```

If you see `Starting server at http://localhost:8080`, congratulations, it didn't crash. On the first try.

### Testing — Because We Pretend To Test 🧪

```bash
# Create a user you'll forget tomorrow
curl -X POST http://127.0.0.1:8080/signup \
  -H "Content-Type: application/json" \
  -d '{"username":"kanha123","email":"kanha@test.com","password":"password123"}'

# Create a Todo you'll never complete
curl -X POST http://127.0.0.1:8080/api/v1/todo \
  -H "Content-Type: application/json" \
  -d '{"id":"1","title":"Learn Rust","description":"Give up and use Node","completed":false,"user_id":"kanha123"}'

# Get Todos to confirm they exist for 5 seconds
curl http://127.0.0.1:8080/api/v1/todo

# Update it, because why not
curl -X PUT http://127.0.0.1:8080/api/v1/todo/1 \
  -H "Content-Type: application/json" \
  -d '{"id":"1","title":"Learn Rust (updated)","description":"Still not done","completed":true,"user_id":"kanha123"}'

# Delete it, like your other side projects
curl -X DELETE http://127.0.0.1:8080/api/v1/todo/1
```

### Known "Features" 🐛

- Hardcoded salt `randomsalt` — Because `env vars` are too secure
- `unwrap()` everywhere — Error handling? Never heard of her.
- No auth, no JWT, no DB — It's not a bug, it's *minimalism*
- `matching()` function is `dead_code` — Just like your GitHub streak
- Prints hashed passwords to stdout — For... debugging... yeah.

### Roadmap — Things We'll Definitely Do (Not) 📝

- [x] Build another Todo app — Done, originality achieved
- [x] Store everything in RAM — So efficient
- [ ] Add a database — Maybe next year
- [ ] Add JWT — Too much typing
- [ ] Add frontend — Lol, no
- [ ] Write tests — The compiler *is* the test

### Contribute? Really? 🤝

1. Fork it (if you have nothing better to do)
2. Add more sarcasm
3. Open a PR we'll pretend to review
4. Get ignored. It's tradition.

---

**Built with ❤️, Sarcasm, and 47 Compiler Warnings by Kanha**

> "It works on my machine" — The official motto
> 
> P.S. If this README offended you, good. That was the point.
