## /equipments
### GET

?applicant_id
?disposaler_id
?type_id
?situation_id

条件にあったequipmentsをarrayで返す

### POST

{
applicant_id
disposaler_id
type_id
equipment_name
..etc
}

登録してidを返す


## /equipments/:id

### GET

idの備品の詳細を返す

### PUT

認証が必要

idの備品のsituationを変更する

### DELETE

idの備品を削除する
認証が必要だった場合と必要ない場合がある

## /login

### POST 
認証する

## /users

### GET

ユーザー一覧を取得する

### POST

認証が必要
ユーザーを追加する

## /users/:user-id

### DELETE

認証が必要

ユーザーを削除する

## /tests

### GET

OKを返す