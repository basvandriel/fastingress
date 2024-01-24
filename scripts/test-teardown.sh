DEFAULT_PROXY_PORT=8001
PROXY_PID=$(lsof -ti :$DEFAULT_PROXY_PORT)

if [ -z $PROXY_PID ]
then
    echo "No proxy PID found, aborting script."
    exit 1;
fi

kill $PROXY_PID