clear
resim reset

# Create account
OP1=$(resim new-account)
export account_add1=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

# Publish the package
PUB=$(resim publish .)
export package_add=$(echo "$PUB" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

# Create multiple tokens
TK1=$(resim new-token-fixed --name Bitcoin --symbol BTC 1000000)
export BTC=$(echo "$TK1" | sed -nr "s/^└─ Resource: ([[:alnum:]_]+)/\1/p")

TK2=$(resim new-token-fixed --name Litecoin --symbol LTC 1000000)
export LTC=$(echo "$TK2" | sed -nr "s/^└─ Resource: ([[:alnum:]_]+)/\1/p")

TK3=$(resim new-token-fixed --name XRP --symbol XRP 1000000)
export XRP=$(echo "$TK3" | sed -nr "s/^└─ Resource: ([[:alnum:]_]+)/\1/p")

TK4=$(resim new-token-fixed --name Dogecoin --symbol DOGE 1000000)
export DOGE=$(echo "$TK4" | sed -nr "s/^└─ Resource: ([[:alnum:]_]+)/\1/p")

CP=$(resim run manifests/env/1_creation.rtm)
export component_add=$(echo "$CP" | sed -nr "s/^└─ Component: ([[:alnum:]_]+)/\1/p")


resim run manifests/env/2_initial_liquidity.rtm

