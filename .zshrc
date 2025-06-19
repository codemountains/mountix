# shellcheck disable=all

# Activate Mise
eval "$(mise activate zsh)"
export PATH="$HOME/.local/share/mise/shims:$PATH"

# If you come from bash you might have to change your $PATH.
# export PATH=$HOME/bin:$HOME/.local/bin:/usr/local/bin:$PATH
# Path to your Oh My Zsh installation.
export ZSH="$HOME/.oh-my-zsh"
export ZSH_CUSTOM="$ZSH/custom"

# Settings to persist history file
export HISTFILE=/root/.history/.zsh_history

# Clone zsh-autosuggestions plugin if it doesn't exist
if [ ! -d $ZSH_CUSTOM/plugins/zsh-autosuggestions ]; then
    git clone https://github.com/zsh-users/zsh-autosuggestions $ZSH_CUSTOM/plugins/zsh-autosuggestions
fi
ZSH_THEME="robbyrussell"

plugins=(git zsh-autosuggestions)

source "$ZSH"/oh-my-zsh.sh
