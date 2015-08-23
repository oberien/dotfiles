# JJ's zshrc
#
# This is brought to you by
#
#     -=[SFT]=-
# proud to be vergil.
#
# Released under GPLv3 or later.



#If not running interactively, don't do anything
[[ $- != *i* ]] && return

#environment variables
export VISUAL="vim"
export EDITOR=$VISUAL
export PAGER="less"
export SHELL="/bin/zsh"

#export GPGKEY="FA2292AF"
export GPSD_UNITS=metric
#export LC_MEASUREMENT=metric
#export SSLKEYLOGFILE=$HOME/.ssl-log

# see man zsh, reports cpu/system/etc usage if running longer then
REPORTTIME=1

# aliases
source ~/.zsh_alias

#general shell options
setopt autocd extended_glob longlistjobs completeinword completealiases hashlistall bash_rematch nohup nobeep
unsetopt beep notify

#command history
HISTFILE=~/.zsh-histfile
HISTSIZE=100000
SAVEHIST=1000000
setopt append_history share_history extended_history histverify histignorespace histignoredups

#directory history
setopt autopushd pushdminus pushdsilent pushdtohome pushdignoredups

setopt interactivecomments

#zrcautoload zmv    # who needs mmv or rename?
#zrcautoload history-search-end

autoload -U colors && colors

#autocompletion (global)
zstyle ':completion::complete:*' use-cache=1
zstyle ':completion:*' auto-description '%d'
zstyle ':completion:*' completer _expand _complete _ignored
zstyle ':completion:*' completions 1
zstyle ':completion:*' expand prefix suffix
zstyle ':completion:*' file-sort modification
zstyle ':completion:*' format "%{$fg[yellow]%}%d%{$reset_color%}"
zstyle ':completion:*' glob 1
zstyle ':completion:*' group-name ''
zstyle ':completion:*' insert-unambiguous true
zstyle ':completion:*' list-colors ${(s.:.)LS_COLORS}
zstyle ':completion:*' list-prompt %SAt %p: Hit TAB for more, or the character to insert%s
zstyle ':completion:*' list-suffixes true
zstyle ':completion:*' matcher-list '' '' '' ''
zstyle ':completion:*' menu select=1
zstyle ':completion:*' original true
zstyle ':completion:*' preserve-prefix '//[^/]##/'
zstyle ':completion:*' select-prompt %SScrolling active: current selection at %p%s
zstyle ':completion:*' squeeze-slashes true
zstyle ':completion:*' substitute 1
zstyle ':completion:*' verbose true
zstyle ':compinstall' filename '/home/jj/.zshrc'


#init the zsh completion
autoload -Uz compinit
compinit

#init the bash compatibility completion
autoload -Uz bashcompinit
bashcompinit


#manpages
#zstyle ':completion:*:manuals'    separate-sections true
#zstyle ':completion:*:manuals.*'  insert-sections   true
#zstyle ':completion:*:man:*'      menu yes select
zstyle ':completion:*:manuals'    separate-sections false
zstyle ':completion:*:manuals.*'  insert-sections   false
zstyle ':completion:*:man:*'      menu yes select

#processes
zstyle ':completion:*:processes'  command 'ps -au$USER'
zstyle ':completion:*:processes-names' command 'ps c -u ${USER} -o command | uniq'

#urls
zstyle ':completion:*:urls' local 'www' 'public_html' '/srv/http'

# host completion, guttenberg'd from grml config
test -r ~/.ssh/known_hosts && _ssh_hosts=(${${${${(f)"$(<$HOME/.ssh/known_hosts)"}:#[\|]*}%%\ *}%%,*}) || _ssh_hosts=()
test -r /etc/hosts && : ${(A)_etc_hosts:=${(s: :)${(ps:\t:)${${(f)~~"$(</etc/hosts)"}%%\#*}##[:blank:]#[^[:blank:]]#}}} || _etc_hosts=()
hosts=(
	$(hostname)
	"$_ssh_hosts[@]"
	"$_etc_hosts[@]"
	8.8.8.8
	2001:4860:4860::8888
	google.com
	127.0.0.1
	::1
	localhost
)
zstyle ':completion:*:hosts' hosts $hosts

#vcs info
autoload -Uz vcs_info
zstyle ':vcs_info:*' enable git svn
zstyle ':vcs_info:*' max-exports 4

zstyle ':vcs_info:*'            check-for-changes true
zstyle ':vcs_info:*'            get-revision    true
zstyle ':vcs_info:*'            stagedstr       "● "
zstyle ':vcs_info:*'            unstagedstr     "# "
zstyle ':vcs_info:(svn|hg):*'   branchformat    "%b:%r"

#%a=action %b=branch %c=stagedstr %u=unstagedstr %i=revision
#%R=basedir %r=reponame %S=subfolder %s=vcsname
zstyle ':vcs_info:*'            formats         "[%r/%b]"       "%c%u"
zstyle ':vcs_info:*'            actionformats   "[%r/%b =>%a]"  "%c%u"



#jj-copy-region-as-kill () {
#	zle copy-region-as-kill
#	print -rn $CUTBUFFER | xsel -i
#}
#zle -N jj-copy-region-as-kill
#
#jj-kill-region () {
#	zle kill-region
#	print -rn $CUTBUFFER | xsel -i
#}
#zle -N jj-kill-region
#
#jj-yank () {
#	CUTBUFFER=$(xsel -o)
#	zle yank
#}
#zle -N jj-yank


#key bindings: emacs style
bindkey -e
typeset -A key
key[Backspace]=${terminfo[kbs]}
key[Home]=${terminfo[khome]}
key[End]=${terminfo[kend]}
key[Insert]=${terminfo[kich1]}
key[Delete]=${terminfo[kdch1]}
key[Up]=${terminfo[kcuu1]}
key[Down]=${terminfo[kcud1]}
key[Left]=${terminfo[kcub1]}
key[Right]=${terminfo[kcuf1]}
key[PageUp]=${terminfo[kpp]}
key[PageDown]=${terminfo[knp]}

bindkey  "${key[Backspace]}" backward-delete-char
bindkey  "${key[Home]}"      beginning-of-line
bindkey  "${key[End]}"       end-of-line
bindkey  "${key[Insert]}"    overwrite-mode
bindkey  "${key[Delete]}"    delete-char
bindkey  "${key[Up]}"        up-line-or-history
bindkey  "${key[Down]}"      down-line-or-history
bindkey  "${key[Left]}"      backward-char
bindkey  "${key[Right]}"     forward-char
bindkey  "${key[PageUp]}"    beginning-of-buffer-or-history
bindkey  "${key[PageDown]}"  end-of-buffer-or-history

if [[ "$TERM" != "xterm" ]]; then
	bindkey "^H" backward-kill-word
fi

#reminder: get keys combos by "cat"
#history searching
bindkey "^[[A"  history-beginning-search-backward
bindkey "^[[B"  history-beginning-search-forward

bindkey "^R"    history-incremental-pattern-search-backward
bindkey "^S"    history-incremental-pattern-search-forward

#special keys for several terminals
bindkey "\e[1~"         beginning-of-line     # Home
bindkey "\e[2~"         quoted-insert         # Ins
bindkey "\e[3~"         delete-char           # Del
bindkey "\e[4~"         end-of-line           # End
bindkey "\e[5~"         beginning-of-history  # PageUp
bindkey "\e[6~"         end-of-history        # PageDown
bindkey "\e[7~"         beginning-of-line     # Home
bindkey "\e[8~"         end-of-line           # End
bindkey "\e[5C"         forward-word
bindkey "\e[5D"         backward-word
bindkey "\e\e[C"        forward-word
bindkey "\e\e[D"        backward-word
bindkey "^[[1;5C"       forward-word
bindkey "^[[1;5D"       backward-word
bindkey "\eOc"          emacs-forward-word
bindkey "\eOd"          emacs-backward-word
bindkey "\e[Z"          reverse-menu-complete # Shift+Tab
bindkey "\eOF"          end-of-line
bindkey "\eOH"          beginning-of-line
bindkey "\e[F"          end-of-line
bindkey "\e[H"          beginning-of-line
bindkey "\eOF"          end-of-line
bindkey "\eOH"          beginning-of-line
bindkey "^[d"           kill-word
bindkey "^[[3^"         kill-word
#bindkey '^[w'           jj-copy-region-as-kill
#bindkey '^W'            jj-kill-region
#bindkey '^Y'            jj-yank



##### DIRCOLOR SECTION
#
#no   NORMAL, NORM          Global default, although everything should be something
#di   DIR                   Directory
#fi   FILE                  Normal file
#ln   SYMLINK, LINK, LNK    Symbolic link. If you set this to ‘target’ instead of a numerical value, the color is as for the file pointed to.
#pi   FIFO, PIPE            Named pipe
#so   SOCK                  Socket
#bd   BLOCK, BLK            Block device
#cd   CHAR, CHR             Character device
#or   ORPHAN                Symbolic link pointing to a non-existent file
#mi   MISSING               Non-existent file pointed to by a symbolic link (visible when you type ls -l)
#ex   EXEC                  Executable file (i.e. has ‘x’ set in permissions)
#su   SETUID                File that is setuid (u+s)
#sg   SETGID                File that is setgid (g+s)
#tw   STICKY_OTHER_WRITABLE Directory that is sticky and other-writable (+t,o+w)
#ow   OTHER_WRITABLE        Directory that is other-writable (o+w) and not sticky
#st   STICKY                Directory with the sticky bit set (+t) and not other-writable
#lc   LEFTCODE, LEFT        Opening terminal code
#rc   RIGHTCODE, RIGHT      Closing terminal code
#ec   ENDCODE, END          Non-filename text
#mh   MULTIHARDLINK         Regular file[s] with more than one link
#ca   CAPABILITY            File with capability
#*.extension           Every file using this extension e.g. *.jpg
#
#
# 0   = default colour
# 1   = bold
# 4   = underlined
# 5   = flashing text
# 6   = no change
# 7   = reverse field
# 8   = hidden (black)
# 9   = strikethrough (cool!)
# 10 - 29 = no change
# 30  = light green
# 31  = red
# 32  = green
# 33  = orange
# 34  = blue
# 35  = purple
# 36  = cyan
# 37  = grey
# 38  = underline
# 39  = no change
# 40  = black background
# 41  = red background
# 42  = green background
# 43  = orange background
# 44  = blue background
# 45  = purple background
# 46  = cyan background
# 47  = grey background
# 90  = dark grey
# 91  = light red
# 92  = light green
# 93  = yellow
# 94  = light blue
# 95  = light purple
# 96  = turquoise
# 100 = dark grey background
# 101 = light red background
# 102 = light green background
# 103 = yellow background
# 104 = light blue background
# 105 = light purple background
# 106 = turquoise background
#
# These can even be combined, so that a parameter like:
# di=5;31;42 => flashing red on green bg

imgc="95"
confc="91"
arc="33"
compc="92"
rawc="47;34"
export LS_COLORS="rs=00:no=00:di=01;36:ln=01;04;33:mh=04:pi=40;33:so=01;35:do=01;35:bd=40;33;01:cd=40;35;01:or=01;09;31;40:su=37;41:sg=30;43:ca=30;41:tw=30;42:ow=34;42;05:st=37;44:ex=32:*.xz=33:*.jpg=$imgc:*.png=$imgc:*.bmp=$imgc:*.gif=$imgc:*.cfg=$confc:*.ini=$confc:*.conf=$confc:*.cnf=$confc:*.pref=$confc:*rc=$confc:*.tar=$arc:*.zip=$arc:*.xz=$compc:*.gz=$compc:*.bz=$compc:*.lzma=$compc:*.gpg=44;93:*.img=$rawc:*.dat=$rawc:*core=31;04:*.bak=32"
#export LS_COLORS='rs=00:no=00:'

#PROMPT="%{$fg[red]%}%n%{$reset_color%}@%{$fg[green]%}%m %{$fg[blue]%}%~ %{$reset_color%}%% "
#RPROMPT="%l%(1j.%{$fg[red]%}[%j]%{$fg[blue]%}.)%(0?..%{$fg[yellow]%}[%?]%{$fg[green]%})%{$fg[blue]%}%2v%{$reset_color%}"

DONTSETRPROMPT=1
setopt prompt_subst

precmd () {
	vcs_info
	#window title:
	print -Pn "\e]0;%n@%M: %~\a"
	psvar[1]=""
	psvar[2]="$vcs_info_msg_0_"
	psvar[3]="$vcs_info_msg_1_"
	psvar[4]="$vcs_info_msg_2_"
	#test $vcs_info_msg_1_ && psvar[4]="$vcs_info_msg_1_" || psvar[4]=`pwd`
}

PROMPT="%B%{$fg[green]%}%n%{$fg[cyan]%}@%{$fg[blue]%}%m%b %{$fg[red]%}%~ %{$fg[yellow]%}%1v%2v%{$reset_color%}%# "
RPROMPT="%3v%4v%{$reset_color%}[%{$fg[yellow]%}%?%{$reset_color%}]%1v%{$fg[blue]%}:%{$fg[red]%}%l%{$reset_color%} "

source /usr/share/nvm/init-nvm.sh
nvm use stable
