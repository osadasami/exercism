class LogLineParser
  def initialize(line)
    @line = line
  end

  def message
    @line.split(':').last.strip
  end

  def log_level
    @line.split(':')
      .first
      .gsub('[', '')
      .gsub(']', '')
      .downcase
  end

  def reformat
    "#{message} (#{log_level})"
  end
end
