class Diamond
	def make_diamond(letter)
		return 'A' if letter == 'A'

		letters = ('A'..letter).to_a
		rows_n = letters.size - 1
	end
end