# MetaSurfAI API Server 🚀

Welcome to the MetaSurfAI API Server repository! This project provides an API server for the MetaSurfAI platform, offering a suite of machine learning and AI capabilities for advanced data analysis and automation.

## Table of Contents 🌐

- [Overview](#overview)
- [Usage](#usage)
- [API Endpoints](#api-endpoints)
- [Contributing](#contributing)
- [License](#license)

## Test Public Api 🦀

- `login` `POST` `https://metasurfai-public-api.fly.dev/v2/login`
- `signup` `POST` `https://metasurfai-public-api.fly.dev/v2/signup`
- Both `signup` and `login` requires `email` and `password` as body params

```json
{
    "email":"nayemtest123@gmail.com",
    "password":"nayemtest123",
}
```

- it will return bearer token, use the token to access the profile endpoint 
- `profile` endpoint `https://metasurfai-public-api.fly.dev/v1/profile` and authorization bearer token

- get sample ads `GET` `https://metasurfai-public-api.fly.dev/v1`
- get ads from db `GET` `https://metasurfai-public-api.fly.dev/v2`
- create ads in db `POST` `https://metasurfai-public-api.fly.dev/v2/ads`

```json
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

- delete ads in db `DELETE` `https://metasurfai-public-api.fly.dev/v2/ads`

```json
{
    "id":"66fde9ec9505c5e64242e982"
}
```


![Funny GIF](https://media.giphy.com/media/HCTfYH2Xk5yw/giphy.gif?cid=790b7611q1p4oj3wetxecugn3qgfrnvda5w17sa8qbpq2kbo&ep=v1_gifs_search&rid=giphy.gif&ct=g)
