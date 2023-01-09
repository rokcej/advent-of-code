using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

namespace Day02 { 

	class Part2 { 

		public static void Main(string[] args) { 
			string[] lines = File.ReadAllLines("input");

			int sum = 0;
			foreach (string line in lines) {
				List<string> words = line.Split('\t').ToList();
				List<int> numbers = words.Select(word => int.Parse(word)).ToList();
				sum += GetDivisionResult(numbers);
			}
			Console.WriteLine(sum);
		}

		private static int GetDivisionResult(List<int> numbers) {
			numbers.Sort();
			for (int i = 0; i < numbers.Count(); ++i) {
				for (int j = i + 1; j < numbers.Count(); ++j) {
					if (numbers[j] % numbers[i] == 0) {
						return numbers[j] / numbers[i];
					}
				}
			}
			return 0;
		}

	}

}
