# How to prepare expiriment in Netlogo
1. Build docker image
    ```docker
        docker build --build-arg MODEL_FOLDER=GameOfLife -t netlogo .
    ```
2. Run container
    ```docker
        docker run netlogo >> netlogo_logs.txt
    ```