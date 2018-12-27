# Game Initiation Protocol

The game initiation protocol is used to setup a game of eridanus between multiple participants. One of the participants takes to role of the initiator that send the initial invites and send updates about changes to all other participants.

## Message Types

### Invite

### Accept

### Update

### Confirm

### Start

```
Bob->Alice: Invite
Bob->Carol: Invite

Alice->Bob: Accept

Bob->Carol: Update
Bob->Alice: Update

Carol->Bob: Accept

Bob->Alice: Update
Bob->Carol: Update

Alice->Bob: Confirm

Carol->Bob: Confirm

Bob->Alice: Start
Bob->Carol: Start
```