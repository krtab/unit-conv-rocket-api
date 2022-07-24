Une API de conversion de quantités de masse entre différentes unités.

Il y a un seul endpoint, `/`, qui accepte une requête POST dont le corps contient un JSON avec trois élements:

- `quantity`: un `number` représentant la quantité numérique
- `unit_from`: l'unité depuis laquelle on veut convertir (celle dans laquelle `quantity` est exprimée)
- `unit_to`: l'unité vers laquelle on veut convertir

Les unités valides sont:

- `g`: gramme
- `kg`: kilogramme
- `lb`: livre
- `metric ton`: une tonne métrique (1000kg)

La réponse retournée sera un objet JSON de deux éléments:

- `quantity`: la quantité convertie
- `unit`: l'unité dans laquelle elle a été convertie (`unit_to` de la requête)

Pour utiliser l'API localement, lancer le serveur avec `cargo run`, et utiliser curl comme client. Il est possible d'utiliser [jq](https://stedolan.github.io/jq/) après un pipe pour pretty-print le JSON retourné.

```
curl -s -X POST 127.0.0.1:8000 \
    -H "Accept: application/json" \
    -d '{"quantity": 17, "unit_from": "lb", "unit_to": "metric ton"}'
```

renverra

```
{"quantity":0.007711070290000001,"unit":"metric ton"}
```

pretty printed en

```
{
  "quantity": 0.007711070290000001,
  "unit": "metric ton"
}
```