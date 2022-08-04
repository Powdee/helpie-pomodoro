# TODO

- [X] **"create new session"** button should be disabled when no session is initialized
- [X] **"clear all"** button should be disabled when no session is initialized
- [ ] create **take a break** functionality
  - [X] add input for minutes
  - [X] add input for seconds
  - [ ] connect **take a break** data with pomodoro store data
- [ ] spike fp-ts for functional programming in typescript
- [ ] add user's store and login/logout button
- [ ] spike axum rust web server
  - [ ] create server folder with localhost server 
    - [ ] **/GET** - pomodoro
    - [ ] **/POST** - pomodoro
    - [ ] **/DELETE/{id}** - pomodoro
  - [ ] connect **OAuth**
  - [ ] connect **web socket**
  - [ ] create a custom session ((start_time + break time) * repeat)
    - [ ] customise each section (e.g: [[25, 5], [20, 5], [15,5] ... ]