class ShellConfig
  def initialize(filename = 'config')
    @entries = {}
    File.open(filename, 'r') { |f|
      f.each_line { |l|
        l.chomp!
        l.strip!
        l.gsub!(/#.*$/, '')
        if l =~ /^([A-Za-z0-9_]+)=(.*?)$/
          key = $1
          value = $2

          value.gsub!(/\$HOME/, ENV['HOME'])

          @entries[key] = value
        end
      }
    }
  end

  def [](key)
    @entries[key]
  end
end
