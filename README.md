# MetaSurfAI API Server

Welcome to the MetaSurfAI API Server repository! This project provides an API server for the MetaSurfAI platform, offering a suite of machine learning and AI capabilities for advanced data analysis and automation.

## Table of Contents

- [Overview](#overview)
- [Usage](#usage)
- [API Endpoints](#api-endpoints)
- [Contributing](#contributing)
- [License](#license)

## Test Public Api

- get sample ads `GET` `https://metasurfai-public-api.fly.dev/v1`
- get ads from db `GET` `https://metasurfai-public-api.fly.dev/v2`
- create ads in db `POST` `https://metasurfai-public-api.fly.dev/v2/createOneAds`

```
{
    "title": "test Nayem",
    "image_url": "https://i.postimg.cc/wTr6w5GD/burger-ads.jpg",
    "view_count": 6,
    "description": "description",
    "posted_by": "posted_by",
    "active": true,
    "max_views": 5,
    "region": "region",
    "token_reward": 5.5
}
```

- delete ads in db `DELETE` `https://metasurfai-public-api.fly.dev/v2/deleteOneAds`

```
{
    "id":"66fde9ec9505c5e64242e982"
}
```

- login with username and password `https://metasurfai-public-api.fly.dev/v1/login`
- Sample profile endpoint `https://metasurfai-public-api.fly.dev/v1/profile?username=nayem`

## Test Locally

- open cmd/terminal
- cd `project/root/dir`
- run`make run`

![Funny GIF](https://media.giphy.com/media/s51QoNAmM6dkWcSC0P/giphy.gif?cid=790b7611q1p4oj3wetxecugn3qgfrnvda5w17sa8qbpq2kbo&ep=v1_gifs_search&rid=giphy.gif&ct=g)
