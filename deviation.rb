file = File.open("inp2.txt")
data = []
file.each do |line|
    data << line.to_f
end
amount = 0
data.each do |number| 
    amount += number
end
average = amount / data.size

amount = 0.0
data.each do |number| 
    amount += (number - average) ** 2
end

puts Math.sqrt(amount / data.size)

