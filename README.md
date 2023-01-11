# goMiner

This go program can allow you to automatically download a monero miner. It automatically detects the os the system is running and runs the compatible miner of the system. 

It uses the runtime.GOOS variable to detect the current operating system and then sets the appropriate command to run either the PowerShell script on windows or shell script on linux. It runs the miner script using the exec.Command function, then uses the .CombinedOutput() method to get the output of the command, and finally, prints the output and error if exist.

It also assumes that you have the required dependencies installed and configured on the system I’ve included the bash script here as well and the Powershell script here, you will still need to configure each of those scripts in order to send back to your wallet.

# The Power$hell script

The Power$hell script
will start by downloading and installing the Visual C++ Redistributable package, that's a common dependency for many programs, this step is skipped on Linux and Mac systems. Then it will download the Monero miner using the Invoke-WebRequest cmdlet. It will unzip the miner using the Expand-Archive cmdlet. Then it will change the sending address for the miner by loading the config.json file into the $config variable, then replacing the YOUR_WALLET_ADDRESS placeholder with the specified Monero address and then saving the changes. Finally, the script will start the miner by running the xmrig.exe with the config file as an argument. Please note that you'll need to replace YOUR_MONERO_WALLET_ADDRESS in the script with your actual Monero wallet address, and also make sure that you have the correct version and architecture of the miner that you need to run.

# The $hell $cript 

The $hell $cript 
will download and extract the Monero miner, download a sample config file, replace the wallet address in the config file with the specified address, and start the miner using the command line. Please note that the script uses the & symbol at the end of the command to run the miner in the background so that the script can continue to run.

# Monero Miner service

Once you have created this file, you can use the command systemctl start monero-miner.service to start the service, systemctl status monero-miner.service to check the status, systemctl stop monero-miner.service to stop it and systemctl enable monero-miner.service to have it start at boot time. Move this file to /etc/systemd/system directory.

