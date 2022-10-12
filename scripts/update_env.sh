# Run this with 'source scripts/update_env.sh' so that conda doesn't get tripped up with running in a subshell
# Cf. https://stackoverflow.com/a/56852970/3893444 for a discussion around why.

# Constants
CONFIG_FILE="environment.yml"

# Make sure we are running in a sourced script instead of an executed one
# shellcheck disable=SC2128
if [[ "$0" = "$BASH_SOURCE" ]]; then
  echo "Please run this script using 'source $0' instead of executing it directly--terminating script"
  exit 1
fi

# Make sure the expected configuration file exists
if ! test -f "$CONFIG_FILE"; then
  echo "The expected configuration file, $CONFIG_FILE, seems to be missing--terminating script"
  return 1
fi

# Get name of current environment
CURRENT_ENV=$(basename "$CONDA_PREFIX")
if [ -z "$CURRENT_ENV" ]; then
  echo "No active conda environment was detected--terminating script"
  return 1
fi
if [ "$CURRENT_ENV" = "anaconda3" ]; then
  echo "Looks like you're currently in the base environment (\"$CURRENT_ENV\")"
  echo "Please run this script in the environment you want to update--terminating script"
  return 1
fi

# Deactivate the environment
# Mamba documentation clearly says to use conda to deactivate and activate environments
# Cf. https://mamba.readthedocs.io/en/latest/user_guide/mamba.html
if ! conda deactivate; then
  echo "Unable to deactivate current environment--terminating script"
  return 1
fi

# Remove the environment
if ! mamba env remove --name "$CURRENT_ENV"; then
  echo "Unable to remove current conda environment--terminating script"
  return 1
fi

# Clean all packages (seems to be needed to make sure this script always executes consistently
if ! mamba clean --all --yes; then
  echo "Unable to clean conda packages--terminating script"
  return 1
fi

# Recreate the environment from the YAML description file
if ! mamba env create -f $CONFIG_FILE; then
  echo "Unable to create new environment from description file--terminating script"
  return 1
fi

# And, activate it (using conda per note above)
if ! conda activate "$CURRENT_ENV"; then
  echo "Unable to activate new environment--terminating script"
  return 1
fi

