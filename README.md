# Quick Commands

```
    cargo sqlx prepare --workspace -- --all-targets
    doctl auth init
    doctl apps create --spec spec.yaml
    doctl apps list
    # You can retrieve your app id using `doctl apps list`
    doctl apps update YOUR-APP-ID --spec=spec.yaml

    DATABASE_URL=YOUR-DIGITAL-OCEAN-DB-CONNECTION-STRING sqlx migrate run

    # Sample Curl
    curl --request POST \
        --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' \
        APP_URL/subscriptions \
        --verbose
```
