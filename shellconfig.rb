class ShellConfig
  def initialize(filename = 'config')
    @entries = {}
    File.open(filename, 'r') { |f|
      f.each_line { |l|
        l.chomp!
        l.strip!
        l.gsub!(/#.*$/, '')
        if l =~ /^([A-Za-z0-9_]+)=(.*?)$/
          @entries[$1] = $2
        end
      }
    }
  end

  def [](key)
    @entries[key]
  end
end
