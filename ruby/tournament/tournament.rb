class Tournament
	def self.tally(input)
		rows = input.split("\n")
		teams = {}

		rows.each do |row|
			team_a, team_b, result = row.split(';')
			teams[team_a] = {wins: 0, drawns: 0, losses: 0, matches_player: 0, points: 0}
			teams[team_b] = {wins: 0, drawns: 0, losses: 0, matches_player: 0, points: 0}
		end

		rows.each do |row|
			team_a, team_b, result = row.split(';')

			if result == 'win'
				teams[team_a][:wins] += 1
				teams[team_a][:points] += 3
				teams[team_b][:losses] += 1
			end

			if result == 'loss'
				teams[team_a][:losses] += 1
				teams[team_b][:wins] += 1
				teams[team_b][:points] += 3
			end

			if result == 'draw'
				teams[team_a][:drawns] += 1
				teams[team_a][:points] += 1
				teams[team_b][:drawns] += 1
				teams[team_b][:points] += 1
			end

			teams[team_a][:matches_player] += 1
			teams[team_b][:matches_player] += 1
		end

		header = 'Team                           | MP |  W |  D |  L |  P'
    
    body = teams
    	.sort_by { |team, stats| team }
    	.sort_by { |team, stats| -stats[:points] }
  		.map { |team, stats| "#{team.ljust(30)} |  #{stats[:matches_player]} |  #{stats[:wins]} |  #{stats[:drawns]} |  #{stats[:losses]} |  #{stats[:points]}" }
  		.join("\n")

		output = <<~TALLY
			#{header}
			#{body}
    TALLY

    output.strip.concat("\n")
	end
end