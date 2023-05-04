# axum API application
Rust Axum REST api application

## Features
- [x] Create REST API with Axum.
- [ ] Basic CRUD calls using SeaORM with MySQL DB.
- [x] Logging.
- [x] JSON request payload validation.
- [ ] Add JWT security.
- [ ] Add roll based JWT.
- [ ] Global and local api error handling.
- [ ] Pagination and Sorting of list for filter.
- [ ] CORS support for API.
- [ ] Custom validation error response.

#### Global Error Handing Response payload
```json
{
  "status": 404,
  "time": "2022-12-25T15:25:35.089z",
  "message": "User not found for id - 2893f9283uo2",
  "debugMessage": "User not found for id - 2893f9283uo2",
  "subErrors": [
    {
      "object": "users",
      "field": "email",
      "rejectedValue": "dummyEmailgmail.com",
      "message": "invalid email address"
    }
  ]
}
```

#### Pagination response structure
```json
{
  "data": [{
    "id": "usr_DwgQxN3gLRX1p0g7bwny1",
    "userName": "john_doe",
    "firstName": "john",
    "lastName": "Doe",
    "email": "john_doe@c12.io"
  },
    {...}
  ],
  "meta": {
    "current_page": 1,
    "page_size": 20,
    "page_count": 12,
    "total_results": 348,
    "search_id": "VE4heV3F5m2Vf0GO_dLhu",
    "search_criteria": "",
    "sort_by": "lastName"
  },
  "_link": {
    "self": {
      "href": "/v1/users?limt=20&offset=40"
    },
    "previous": {
      "href": "/v1/users?limit=20&offset=20"
    },
    "first": {
      "href": "/v1/users?limit=20&offset=0"
    },
    "next": {
      "href": "/v1/users?limit=20&offset=60"
    },
    "last": {
      "href": "/v1/users?limit=20&offset=120"
    }
  }
}
```

### References
- Actix api [example](https://github.com/b-palaniappan/actix-api)
