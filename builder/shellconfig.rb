class ShellConfig
  def initialize(filename = nil)
    @entries = {}

    if filename.nil?
      script_dir = File.dirname(File.dirname(__FILE__))
      filename = File.join(script_dir, 'config')
    end

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
