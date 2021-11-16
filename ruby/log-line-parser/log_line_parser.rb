class LogLineParser
  attr_reader :log_level, :message
  def initialize(line)
    parsed = /\[(?<log_level>[A-Z]+)\]:\s*(?<message>[\w|\s]+)/.match(line)
    
    @log_level = parsed[:log_level].downcase
    @message = parsed[:message].strip
  end

  def reformat
    "#{message} (#{log_level})"
  end
end
