::copy keys and build docker
set serverKeys=C:\DockerData\BantamweightRustDocker
set devKeys=C:\DockerData\DevKeys

::copy keys
xcopy /s %serverKeys% keys
xcopy /s %devKeys% dev_keys
takeown /f . /r /d y
takeown /f dev_keys /r /d y
takeown /f keys /r /d y

::build docker
docker compose build