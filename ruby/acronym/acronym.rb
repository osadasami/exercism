class Acronym
  def self.abbreviate(phrase)
    phrase.scan(/\w+/).map(&:chr).join.upcase
  end
end