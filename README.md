# Rstore

A Rust-based artifact repository (still in development)

| usage | name |
|--|--|
|http frame|axum|
|orm|disel|
|s3|aws-sdk-s3|
|log|log4rs|
|config|config-rs|
|http client|reqwest|

## Target

Provide support for java(maven), golang(goproxy), nodejs(npm)

## usage 

### Maven
Add the private repository setting in the `pom.xml` file

A example 
```xml
        <repository>
            <id>hust</id>
            <name>hust</name>
<!--            <url>http://211.67.25.90:8082/repository/maven-snapshots/</url>-->
            <url>http://127.0.0.1:3000/packages/maven/</url>
            <snapshots>
                <enabled>true</enabled>
                <updatePolicy>always</updatePolicy>
            </snapshots>
        </repository>
```

Maven deploy

![image.png](https://s2.loli.net/2023/05/01/pOghcXzTytEAWqU.png)



## config 

The config file is in the `rstore.toml`, it should be able to be specified.