package com.kokoadev.ndkwithrust

import android.os.Bundle
import android.util.Log
import androidx.appcompat.app.AppCompatActivity
import com.kokoadev.ndkwithrust.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        // Example of a call to a native method
//        binding.sampleText.text = stringFromJNI()
//        binding.sampleText.text = helloNameFromJNI("kokoa")
        binding.sampleText.text = addNumberFromJNI(1,2).toString()

        val result = sortIntArrayFromJNI(listOf(5,4,3,2,1).toIntArray())
        binding.sampleText.text = result.toList().joinToString()
    }

    /**
     * A native method that is implemented by the 'ndkwithrust' native library,
     * which is packaged with this application.
     */
    external fun stringFromJNI(): String

    external fun helloNameFromJNI(input: String): String
    external fun addNumberFromJNI(input1: Int, input2: Int): Int
    external fun sortIntArrayFromJNI(input: IntArray): IntArray

    companion object {
        // Used to load the 'ndkwithrust' library on application startup.
        init {
            System.loadLibrary("hello")
        }
    }
}