---
name: Add endpoint template
about: Suggest to add a fake server endpoint
title: ''
labels: no endpoint
assignees: ''

---

<!-- I will eventually add every endpoint, but if you want to speed up the development of a specific endpoint, you can make this issue! -->

**Endpoint:**
https://core.telegram.org/bots/api#yourendpoint

**This endpoint has to simulate:**
__A description of what you want to see this endpoint do in a testing environment__
__For example:__
This endpoint has to return a fake user on request
<!-- Important!!!! Remember that this crate doesn't have access to telegram! If a request is made to get a user, this crate can't get the real user, it doesn't know anything about it! So, it has to fake it. BUT this crate emulates messages as if they were actually sent, so you can ask for something like "This endpoint has to change this message in that way" -->
