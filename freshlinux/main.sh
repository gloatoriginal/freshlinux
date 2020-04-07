sudo apt-get update -y
echo "Preparing to Install nodejs" using apt-get
sudo apt-get install nodejs -y
sleep 2

echo "Preparing to Install build-essential" using apt-get
sudo apt-get install build-essential -y
sleep 2

echo "Preparing to Install python3-pip" using apt-get
sudo apt-get install python3-pip -y
sleep 2

echo "Preparing to Install default-jre" using apt-get
sudo apt-get install default-jre -y
sleep 2

sudo apt-get install default-jdk -y
echo "Preparing to Install snapd" using apt-get
sudo apt-get install snapd -y
sleep 2

echo "Preparing to Install curl" using apt-get
sudo apt-get install curl -y
sleep 2

echo "Preparing to Install git" using apt-get
sudo apt-get install git -y
sleep 2

echo "Preparing to Install code --classic" using snap
sudo snap install code --classic -y
sleep 2

